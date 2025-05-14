#![no_std]
#![no_main]

extern crate alloc;

mod dos_tests;

use rust_dos::*;
use crate::dos_tests::allocator_test::allocator_test;
use crate::dos_tests::datetime::datetime_test;
use crate::dos_tests::file::file_read_test;
use crate::dos_tests::cooperative_multitasking_test::cooperative_multitasking_test;

entry!(main);

fn main() {
    allocator_test();
    file_read_test();
    datetime_test();
    cooperative_multitasking_test();
    println!("Hello, World!");
}
