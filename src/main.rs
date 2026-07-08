#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

mod dos_tests;

use crate::dos_tests::allocator_test::allocator_test;
use crate::dos_tests::cooperative_multitasking_test::cooperative_multitasking_test;
use crate::dos_tests::datetime::datetime_test;
use crate::dos_tests::file::file_read_test;
use rust_dos::*;

entry!(main);

fn main() {
    let args = dos::env::args();

    if args.len() >= 1 && args[0] == "test" {
        allocator_test();
        file_read_test();
        datetime_test();
        cooperative_multitasking_test();
    } else {
        println!("Hello, World!");
    }
}




