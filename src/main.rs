#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe {
        asm!("mov $$0x2, %ah
              mov $$0x41,%dl
              int $$0x21
              int $$0x20"
              ::: "eax", "edx");
    }
    loop {}
}
