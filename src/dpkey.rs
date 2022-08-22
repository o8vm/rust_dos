// dpkey.rs: Dos Play with KEY board controler module
//  system scan code to ascii code demo
//
// Details:
//  When you hit the keyboard from A to ' ,
//  this module reads the system scan code,
//  converts it, and displays the ASCII code.
//  Enjoy!
//
// Dependency:
//  rust_dos::dos::kbc.rs
//
// Note:
//  As far as I've tested, qemu doesn't support this operation, so play with bochs.

use crate::dos::kbc::*;

const MOD_ALT: u8 = 8;
const MOD_CTRL: u8 = 4;
const MOD_SHIFT: u8 = 2;
const MOD_CAPS: u8 = 1;

const ESC: u8 = 0x01;
const CTRL: u8 = 0x1D;
const KEY_A: u8 = 0x1E;
const KEY_SQ: u8 = 0x28;
const SHIFT: u8 = 0x2A;
const ALT: u8 = 0x38;
const CAPS: u8 = 0x3A;

static MAP_PLAIN: [u8; 11] = [
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b':',
];
static MAP_CTRL: [u8; 11] = [
    0x01, 0x13, 0x04, 0x06, 0x07, 0x08, 0x0A, 0x0B, 0x0C, 0x7F, 0x7F,
];
static MAP_SHIFT: [u8; 11] = [
    b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"',
];
static MAP_ALT: [u8; 11] = [
    0x81, 0x93, 0x84, 0x86, 0x87, 0x88, 0x8A, 0x8B, 0x8C, 0xFF, 0xFF,
];

pub fn keymap() {
    let mut ret: u8;
    let mut up: u8;
    let mut ch: u8;
    let mut map: &[u8];

    let mut modifier: u8 = 0;
    let mut capslock: u8 = 0;

    kbc_command(KBC_GET_CTRL);
    ret = kbc_read();
    kbc_command(KBC_SET_CTRL);
    ret &= !KBC_INT_KEY;
    kbc_write(ret);

    loop {
        ret = kbc_read();
        up = ret & 0x80;

        ret &= 0x7F;
        match ret {
            ESC => break,
            ALT => {
                if up != 0 {
                    modifier &= !MOD_ALT;
                } else {
                    modifier |= MOD_ALT;
                }
                continue;
            }
            CTRL => {
                if up != 0 {
                    modifier &= !MOD_CTRL;
                } else {
                    modifier |= MOD_CTRL;
                }
                continue;
            }
            SHIFT => {
                if up != 0 {
                    modifier &= !MOD_SHIFT;
                } else {
                    modifier |= MOD_SHIFT;
                }
                continue;
            }
            CAPS => {
                if up != 0 {
                    modifier &= !MOD_CAPS;
                    capslock ^= 1;
                } else {
                    modifier |= MOD_CAPS;
                }
                continue;
            }
            KEY_A..=KEY_SQ => {
                if up != 0 {
                    continue;
                } else {
                    if modifier & MOD_SHIFT != 0 {
                        map = &MAP_SHIFT;
                    } else if modifier & MOD_CTRL != 0 {
                        map = &MAP_CTRL;
                    } else if modifier & MOD_ALT != 0 {
                        map = &MAP_ALT;
                    } else {
                        map = &MAP_PLAIN;
                    }
                }
                ch = if let Some(&num) = map.get((ret - KEY_A) as usize) {
                    num
                } else {
                    b'X'
                };
                if capslock == 1 {
                    if modifier & MOD_SHIFT == 0 {
                        if ch >= b'a' && ch <= b'z' {
                            ch -= 0x20;
                        }
                    } else {
                        if ch >= b'A' && ch <= b'Z' {
                            ch += 0x20;
                        }
                    }
                }
            }
            _ => continue,
        }
        print!("{:02X}", ch);
        print!(" ");
    }

    kbc_command(KBC_GET_CTRL);
    ret = kbc_read();
    kbc_command(KBC_SET_CTRL);
    ret |= KBC_INT_KEY;
    kbc_write(ret);
}
