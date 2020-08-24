#![no_std]
#![no_main]

use panic_rtt_target as _;
use stm32f4xx_hal as hal;

use cortex_m_rt::entry;
use rtt_target::rprintln;
use hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    loop {
        rprintln!("Hello, world!");
    }
}
