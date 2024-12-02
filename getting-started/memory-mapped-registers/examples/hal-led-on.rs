#![no_main]
#![no_std]

use panic_halt as _; // panic handler
use cortex_m_rt::entry;
// use stm32f4 as pac;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Create handles for the device and core peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Create an the LED abstraction. On the Nucleo-401RE it's connected to pin PA5.
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        led.set_high();
    }
}