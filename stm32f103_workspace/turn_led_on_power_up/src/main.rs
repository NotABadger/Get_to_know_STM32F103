#![no_main]
#![no_std]
use cortex_m_rt::entry;
use panic_halt as _;

use hall_lib::add;
#[entry]
fn main() ->! {
    let nr = 5;
    let nr_two = 10;
    loop {}
}

