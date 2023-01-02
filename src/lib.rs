#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod dos;
pub mod dpkey;
extern crate rlibc;
extern crate alloc;

use crate::dos::allocator::GLOBAL_ALLOCATOR;
use crate::dos::cooperative_multitasking::TASKING;

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe {
        GLOBAL_ALLOCATOR.init();
        TASKING.init(); // Relies on the allocator
    }
    extern "Rust" {
        fn main() -> ();
    }
    unsafe {
        main();
    }
    dos::exit(0);
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;
            f()
        }
    };
}