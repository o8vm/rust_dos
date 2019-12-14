#[macro_use]
pub mod console;
pub mod panic;
pub mod io;
// pub mod kbc;

pub fn exit(rt: u8) -> ! {
    unsafe {
        asm!("mov $$0x4C, %ah
              int $$0x21"
              :
              : "{al}"(rt)
              : "eax");
    }
    loop {}
}