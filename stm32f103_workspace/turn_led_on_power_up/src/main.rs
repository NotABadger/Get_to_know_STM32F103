#![no_main]
#![no_std]
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() ->! {
    //Unlimited power on LED!!!
    //Note, pin A3 had LED annode (Drive high for light)
    //how though?
    loop {}
}

