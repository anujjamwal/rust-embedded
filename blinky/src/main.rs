#![cfg_attr(not(test), no_std)]
#![no_main]

use embed::entry;

entry!(main);

pub fn main() -> ! {
    let _x = 42;

    loop {}
}
