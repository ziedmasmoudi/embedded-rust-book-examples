#![no_std]
#![no_main]

use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use core::fmt::Write; // allows use to use the WriteLn! macro for easy printing
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // let cp = pac::CorePeripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let tx_pin = gpioa.pa2.into_alternate();

    let mut tx = Serial::tx(
        dp.USART2,
        tx_pin,
        Config::default()
            .baudrate(115200.bps())
            .wordlength_8()
            .parity_none(),
        &clocks,
    )
    .unwrap();

    loop {
        writeln!(tx, "Hello, World!\r\n").unwrap();
    }
}