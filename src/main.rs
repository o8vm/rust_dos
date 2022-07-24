#![no_std]
#![no_main]

use rust_dos::*;

entry!(main);

fn main() {
    let test_file = dos::file::File::open("test.txt").unwrap();
    let mut buffer = [0; 20];
    let bytes_read = test_file.read(&mut buffer).unwrap();
    println!("{} bytes read", bytes_read);
    println!("{}", core::str::from_utf8(&buffer).unwrap());
    match test_file.close() {
        Ok(_) => println!("File closed"),
        Err(_) => println!("Error closing file")
    }

    let test_file = dos::file::File::open("C:\\AUTOEXEC.BAT").unwrap();
    let mut buffer = [0; 100];
    let bytes_read = test_file.read(&mut buffer).unwrap();
    println!("{} bytes read", bytes_read);
    println!("{}", core::str::from_utf8(&buffer).unwrap());

    match test_file.close() {
        Ok(_) => println!("File closed"),
        Err(_) => println!("Error closing file")
    }

    println!("Hello, World!");
    // println!("Hit any Key, please.");
    // dpkey::keymap();
}
