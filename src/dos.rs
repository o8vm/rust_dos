#[macro_use]
pub mod console;
pub mod panic;
pub mod io;
pub mod kbc;
use core::arch::asm;

pub fn exit(rt: u8) -> ! {
    unsafe {
        asm!("mov ah, 0x4C",
             "int 0x21", in("al") rt);
    }
    loop {}
}
