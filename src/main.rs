#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use rust_dos::*;

entry!(main);

fn main() {
    println!("Hello, world!");
    let heap_string = Box::new("hello_from_heap");
    println!("{}", *heap_string);
    let heap_char = Box::new('a');
    println!("{}", *heap_char);
    let mut heap_int = Box::new(42);
    *heap_int = 43;
    println!("{}", heap_int.to_string());
    //println!("{}", heap_string);
    let simple_string: String = "hello_from_heap_stored_string".to_string();
    println!("{}", simple_string);
    {
        let mut konnichiwa_str: String = "konnichiwa ".to_string();
        let sekai_str: String = "sekai".to_string();
        konnichiwa_str.push_str(&sekai_str);
        println!("{}", konnichiwa_str);
    }

    println!("End of main()");

    //println!("Hit any Key, please.");
    //dpkey::keymap();
}