#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use rp2040_panic_usb_boot as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    // Do nothing
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
