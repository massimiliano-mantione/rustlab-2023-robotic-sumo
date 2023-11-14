#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as InterruptHandlerUsb};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use rp2040_panic_usb_boot as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandlerUsb<USB>;
});

pub static PING: Signal<CriticalSectionRawMutex, usize> = Signal::new();
pub static PONG: Signal<CriticalSectionRawMutex, usize> = Signal::new();

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn ping_pong() {
    loop {
        let received = PING.wait().await;
        log::info!("ping RECV {}", received);
        PONG.signal(received);
        log::info!("pong SENT {}", received);
    }
}

#[embassy_executor::task]
async fn pong_receiver() {
    loop {
        let received = PONG.wait().await;
        log::info!("pong RECV {}", received);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Init USB logger
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Spawn ping pong tasks
    spawner.spawn(ping_pong()).unwrap();
    spawner.spawn(pong_receiver()).unwrap();

    // Send pings
    let mut counter = 1;
    loop {
        PING.signal(counter);
        log::info!("ping SENT {}", counter);
        counter += 1;
        Timer::after(Duration::from_millis(500)).await;
    }
}
