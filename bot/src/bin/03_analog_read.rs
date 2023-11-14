#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc, InterruptHandler as InterruptHandlerAdc};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as InterruptHandlerUsb};
use embassy_time::{Duration, Timer};
use rp2040_panic_usb_boot as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandlerUsb<USB>;
    ADC_IRQ_FIFO => InterruptHandlerAdc;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Init USB logger
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Init ADC
    let mut adc = Adc::new(p.ADC, Irqs, ConfigAdc::default());

    // Init input pins
    let mut gp26 = Channel::new_pin(p.PIN_26, Pull::None);

    // Read pin
    loop {
        let gp26_level = adc.read(&mut gp26).await.unwrap();
        log::info!("GP26: {}", gp26_level);
        Timer::after(Duration::from_millis(200)).await;
    }
}
