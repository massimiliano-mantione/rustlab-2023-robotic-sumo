#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_net::{Config as ConfigNet, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
use embassy_rp::pio::{InterruptHandler as InterruptHandlerPio, Pio};
use embassy_rp::usb::{Driver, InterruptHandler as InterruptHandlerUsb};
use embassy_time::{Duration, Timer};
use rp2040_panic_usb_boot as _;
use static_cell::make_static;

const WIFI_SSID: &'static str = include_str!("../WIFI_SSID.txt");
const WIFI_SECRET: &'static str = include_str!("../WIFI_SECRET.txt");

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandlerUsb<USB>;
    PIO0_IRQ_0 => InterruptHandlerPio<PIO0>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Init USB logger
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Use cyw43 firmware
    let fw = include_bytes!("../../deps/cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../deps/cyw43-firmware/43439A0_clm.bin");

    // Init cyw43
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );
    let state = make_static!(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // Generate random seed
    let seed = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guarenteed to be random.

    // Init network stack
    let config = ConfigNet::dhcpv4(Default::default());
    let stack = &*make_static!(Stack::new(
        net_device,
        config,
        make_static!(StackResources::<2>::new()),
        seed
    ));
    spawner.spawn(net_task(stack)).unwrap();

    // Join wifi network
    log::info!(
        "Joining access point {} (link up: {})",
        WIFI_SSID,
        stack.is_link_up()
    );
    loop {
        match control.join_wpa2(WIFI_SSID, WIFI_SECRET).await {
            Ok(_) => break,
            Err(err) => {
                log::info!("join failed with status={}", err.status);
            }
        }
    }

    // Wait...
    loop {
        log::info!("Link is up: {}", stack.is_link_up());
        Timer::after(Duration::from_secs(1)).await;
    }
}
