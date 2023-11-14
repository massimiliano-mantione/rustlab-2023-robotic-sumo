#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc, InterruptHandler as InterruptHandlerAdc};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_rp::usb::{Driver, InterruptHandler as InterruptHandlerUsb};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
use rp2040_panic_usb_boot as _;
use static_cell::make_static;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandlerUsb<USB>;
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    ADC_IRQ_FIFO => InterruptHandlerAdc;
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

const PWN_DIV_INT: u8 = 250;
const PWM_TOP: u16 = 10000;

fn pwm_config(duty_a: u16, duty_b: u16) -> PwmConfig {
    let mut c = PwmConfig::default();
    c.invert_a = false;
    c.invert_b = false;
    c.phase_correct = false;
    c.enable = true;
    c.divider = PWN_DIV_INT.to_fixed();
    c.compare_a = duty_a;
    c.compare_b = duty_b;
    c.top = PWM_TOP;
    c
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

    // Init pins
    let gp0 = Input::new(p.PIN_0, Pull::None);
    let gp1 = Input::new(p.PIN_1, Pull::None);
    let mut adc = Adc::new(p.ADC, Irqs, ConfigAdc::default());
    let mut gp26 = Channel::new_pin(p.PIN_26, Pull::None);
    let gp27 = Input::new(p.PIN_27, Pull::None);
    let mut pwm_1 = Pwm::new_output_ab(p.PWM_CH1, p.PIN_2, p.PIN_3, pwm_config(0, 0));
    let mut pwm_2 = Pwm::new_output_ab(p.PWM_CH3, p.PIN_6, p.PIN_7, pwm_config(0, 0));

    // Test hardware
    loop {
        let l0 = gp0.is_high();
        let l1 = gp1.is_high();
        let l27 = gp27.is_high();
        let l26 = adc.read(&mut gp26).await.unwrap();

        let (c1a, c1b, c2a, c2b) = match (l0, l1) {
            (false, false) => (3500, 0u16, 3500, 0u16),
            (false, true) => (3500, 0u16, 0u16, 3500),
            (true, false) => (0u16, 3500, 3500, 0u16),
            (true, true) => (0u16, 0u16, 0u16, 0u16),
        };
        let c1 = pwm_config(c1a, c1b);
        let c2 = pwm_config(c2a, c2b);
        pwm_1.set_config(&c1);
        pwm_2.set_config(&c2);
        control.gpio_set(0, l27).await;

        log::info!("l0 {} l1 {} l27 {} l26 {}", l0, l1, l27, l26);
        Timer::after(Duration::from_millis(50)).await;
    }
}
