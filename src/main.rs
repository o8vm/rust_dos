#![no_std]
#![no_main]


extern crate alloc;

mod dos_tests;

use rust_dos::*;
use crate::dos_tests::allocator_test::allocator_test;
use crate::dos_tests::datetime::datetime_test;
use crate::dos_tests::file::file_read_test;
use crate::dos_tests::file::file_attribute_test;
use crate::dos_tests::file::directory_test;

entry!(main);

fn main() {
    allocator_test();
    file_read_test();
    file_attribute_test();
    directory_test();
    datetime_test();
}
