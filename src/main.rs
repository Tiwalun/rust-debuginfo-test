#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

    loop {
        delay(1000);
    }
}


fn delay(ms: u16) {
    ms + 1;
}