#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use core::str;
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
use embassy_rp::pio::{InterruptHandler as InterruptHandlerPio, Pio};
use embassy_rp::usb::{Driver, InterruptHandler as InterruptHandlerUsb};
use embassy_time::{Duration, Timer};
use rp2040_panic_usb_boot as _;
use static_cell::make_static;

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
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // Scan wifi access points
    loop {
        log::info!("Starting wifi scan");

        let mut scanner = control.scan().await;
        loop {
            match select(scanner.next(), Timer::after(Duration::from_secs(1))).await {
                Either::First(bss) => {
                    if let Some(bss) = bss {
                        if let Ok(ssid_str) = str::from_utf8(&bss.ssid) {
                            log::info!("scanned {} == {:?}", ssid_str, bss.bssid);
                        } else {
                            log::info!("bad bss, cannot log");
                        }
                    } else {
                        log::info!("scanning terminated");
                        break;
                    }
                }
                Either::Second(_) => {
                    log::info!("scanning...");
                }
            }
        }

        Timer::after(Duration::from_secs(1)).await;
    }
}
