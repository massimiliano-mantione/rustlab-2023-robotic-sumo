#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as InterruptHandlerUsb};
use embassy_time::{Duration, Timer};
use rp2040_panic_usb_boot as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandlerUsb<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

fn level2str(l: Level) -> &'static str {
    match l {
        Level::Low => "LO",
        Level::High => "HI",
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Init USB logger
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Init input pins
    let gp0 = Input::new(p.PIN_0, Pull::None);
    let gp1 = Input::new(p.PIN_1, Pull::None);

    // Read pins
    loop {
        let gp0_level = gp0.get_level();
        let gp1_level = gp1.get_level();
        log::info!(
            "GP0: {}, GP1: {}",
            level2str(gp0_level),
            level2str(gp1_level)
        );
        Timer::after(Duration::from_millis(200)).await;
    }
}
