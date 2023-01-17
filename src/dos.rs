#[macro_use]
pub mod console;
pub mod allocator;
pub mod io;
pub mod kbc;
pub mod datetime;
pub mod file;
pub mod error_code;
pub mod panic;
pub mod math;
use core::arch::asm;

pub use alloc::string::String as String;
pub use alloc::boxed::Box as Box;
pub use alloc::vec::Vec as Vec;
pub use alloc::vec as vec;

pub fn exit(rt: u8) -> ! {
    unsafe {
        asm!("mov ah, 0x4C",
             "int 0x21", in("al") rt);
    }
    loop {}
}
