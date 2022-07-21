#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod dos;
pub mod dpkey;
extern crate rlibc;

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
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
