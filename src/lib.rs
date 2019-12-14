#![no_std]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section=".startup"]
#[no_mangle]
fn _start() -> ! {
    extern "Rust" {
        fn main() -> !;
    }
    unsafe {
        main()
    }
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    }
}