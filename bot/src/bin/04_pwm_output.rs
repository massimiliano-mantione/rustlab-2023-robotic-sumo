#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_rp::usb::{Driver, InterruptHandler as InterruptHandlerUsb};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
use rp2040_panic_usb_boot as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandlerUsb<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
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

const MAX_DUTY: u16 = 3500;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Init USB logger
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Init PWM pins
    let mut pwm_1 = Pwm::new_output_ab(p.PWM_CH1, p.PIN_2, p.PIN_3, pwm_config(0, 0));
    let mut pwm_2 = Pwm::new_output_ab(p.PWM_CH3, p.PIN_6, p.PIN_7, pwm_config(0, 0));

    // Use PWM pins
    let mut counter = 0;
    loop {
        let (duty_a, duty_b) = match counter % 4 {
            1 => (MAX_DUTY, 0),
            2 => (0, MAX_DUTY),
            3 => (MAX_DUTY, MAX_DUTY),
            _ => (0, 0),
        };

        log::info!("A: {}, B: {}", duty_a, duty_b);

        let c1 = pwm_config(duty_a, duty_b);
        let c2 = pwm_config(duty_a, duty_b);

        pwm_1.set_config(&c1);
        pwm_2.set_config(&c2);

        counter = (counter + 1) % 4;

        Timer::after(Duration::from_millis(500)).await;
    }
}
