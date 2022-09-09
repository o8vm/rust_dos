#[macro_use]
pub mod console;
pub mod allocator;
pub mod io;
pub mod kbc;
pub mod file;
pub mod error_code;
pub mod panic;
use core::arch::asm;

pub fn exit(rt: u8) -> ! {
    unsafe {
        asm!("mov ah, 0x4C",
             "int 0x21", in("al") rt);
    }
    loop {}
}
