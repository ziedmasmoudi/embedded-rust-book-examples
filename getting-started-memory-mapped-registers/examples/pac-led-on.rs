#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use stm32f4::stm32f401;

#[entry]
fn main() -> ! {
    let peripherals = stm32f401::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let rcc = &peripherals.RCC;

    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
    gpioa.moder.write(|w| w.moder5().output());  
    gpioa.odr.modify(|_, w| w.odr5().set_bit());

    loop {}
}
