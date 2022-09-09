#![no_std]
#![no_main]

use core::convert::TryFrom;
use rust_dos::*;
use rust_dos::dos::error_code::ErrorCode;

entry!(main);

fn main() {
    println!("Hello, World!");
    println!("{}", ErrorCode::from_u8(ErrorCode::NetworkDeviceFault.to_u8()).unwrap());
    let error_code: u8 = ErrorCode::NetworkDeviceFault.into();
    println!("{}", ErrorCode::try_from(error_code).unwrap());
    let err: Result<(), ErrorCode> = Err(ErrorCode::MemoryControlBlockDestroyed);
    err.unwrap();
    // println!("Hit any Key, please.");
    // dpkey::keymap();
}
