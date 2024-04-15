#![no_main]
#![no_std]

use stm32f1::stm32f103;
use my_app as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    my_app::exit()
}
