use super::io::{inb, outb};

// 8042 keyboard controller definition
pub const KBC_CTRL: usize = 0x64;
pub const KBC_DATA: usize = 0x60;
pub const IO_WAIT: usize = 0x80;

// status
pub const KBC_OBF: u8 = 0x01;
pub const KBC_IBF: u8 = 0x02;
pub const KBC_BUSY: u8 = KBC_OBF | KBC_IBF;

// 8042 command byte
pub const KBC_GET_CTRL: u8 = 0x20;
pub const KBC_SET_CTRL: u8 = 0x60;

// 8042 control mode
pub const KBC_DO_XLAT: u8 = 0x40;
pub const KBC_DIS_MOUSE: u8 = 0x20;
pub const KBC_DIS_KEY: u8 = 0x10;
pub const KBC_SYS_FLAG: u8 = 0x04;
pub const KBC_INT_MOUSE: u8 = 0x02;
pub const KBC_INT_KEY: u8 = 0x01;

// Device command byte
pub const KBC_WRITE_LED: u8 = 0xED;
pub const KBC_SET_REPEAT: u8 = 0xF3;
pub const KBC_ACK: u8 = 0xFA;

// 8259 programmable interrupt controller
pub const PIC_MIMR: usize = 0x21;
pub const PIC_IMR_KEY: u8 = 0x2;

// read the status register of 8042 KBC
//  return data
pub fn kbc_status() -> u8 {
    inb(KBC_CTRL)
}

pub fn kbc_command(cmd: u8) {
    loop {
        if kbc_status() & KBC_IBF == 0 {
            break;
        }
        inb(IO_WAIT);
    }
    outb(cmd, KBC_CTRL);
}

pub fn kbc_write(data: u8) {
    inb(KBC_DATA);
    loop {
        if kbc_status() & KBC_BUSY == 0 {
            break;
        }
        inb(IO_WAIT);
    }
    inb(IO_WAIT);
    outb(data, KBC_DATA);
}

pub fn kbc_read() -> u8 {
    loop {
        if kbc_status() & KBC_OBF != 0 {
            break;
        }
        inb(IO_WAIT);
    }
    inb(IO_WAIT);
    inb(KBC_DATA)
}

pub fn disable_keyint() {
    let mut imr: u8;
    imr = inb(PIC_MIMR);
    imr |= PIC_IMR_KEY;
    outb(imr, PIC_MIMR);
}

pub fn enable_keyint() {
    let mut imr: u8;
    imr = inb(PIC_MIMR);
    imr &= !PIC_IMR_KEY;
    outb(imr, PIC_MIMR);
}
