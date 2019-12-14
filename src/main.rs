#![feature(asm)]
#![no_std]
#![no_main]

use rust_dos::entry;

entry!(main);

fn main() -> ! {
    unsafe {
        asm!("mov $$0x2, %ah
              mov $$0x41,%dl
              int $$0x21
              int $$0x20"
              ::: "eax", "edx");
    }
    loop {}
}