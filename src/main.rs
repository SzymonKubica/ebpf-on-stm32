#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};


#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOB.split();
        let mut led0 = gpioa.pb0.into_push_pull_output();
        let mut led1= gpioa.pb7.into_push_pull_output();
        let mut led2 = gpioa.pb14.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // On for 1s, off for 1s.
            led0.toggle();
            delay.delay_ms(1000_u32);
            led1.toggle();
            delay.delay_ms(1000_u32);
            led2.toggle();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
