#![no_std]
#![no_main]
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use rust_dos::*;

entry!(main);

fn main() {
    println!("Hello, World!");
    let heap_string = Box::new("hello_from_heap");
    println!("{}", heap_string);
    let heap_int = Box::new(42);
    println!("{}", heap_int.to_string());
    let simple_string: String = "hello_from_heap_stored_string".to_string();
    println!("{}", simple_string);

    //println!("Hit any Key, please.");
    //dpkey::keymap();
}
