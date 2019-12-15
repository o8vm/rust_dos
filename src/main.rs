#![no_std]
#![no_main]

use rust_dos::*;

entry!(main);

fn main() {
    println!("Hello, World!");
    // println!("Hit any Key, please.");
    // dpkey::keymap();
}