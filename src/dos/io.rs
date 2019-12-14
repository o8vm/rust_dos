pub fn inb(port: usize) -> u8 {
    let mut ret: u8;
    unsafe {
        asm!("xorl %eax, %eax
              inb  %dx,  %al"
             : "={al}"(ret)
             : "{dx}"(port)
            );
    }
    ret
}

pub fn inw(port: usize) -> u16 {
    let mut ret: u16;
    unsafe {
        asm!("xorl %eax, %eax
              inw  %dx,  %ax"
             : "={ax}"(ret)
             : "{dx}"(port)
            );
    }
    ret
}

pub fn outb(data: u8, port: usize) {
    unsafe {
        asm!("outb %al, %dx"
             :
             : "{al}"(data), "{dx}"(port)
            );
    }
}

pub fn outw(data: u16, port: usize) {
    unsafe {
        asm!("outw %ax, %dx"
             :
             : "{ax}"(data), "{dx}"(port)
            );
    }
}