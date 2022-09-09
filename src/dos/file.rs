use core::arch::asm;
use core::cmp::min;

extern crate rlibc;

#[allow(dead_code)]
pub struct File {
    handle: u16,
}

// TODO: return Err(error_code) instead of Err()
#[allow(dead_code)]
#[allow(unused_assignments)]
impl File {
    pub fn open(filename: &str) -> Result<Self, ()> {
        let mut is_open_success: u16 = 1; // 0: success, 1: fail
        let mut error_code_or_handle: u16 = 0;
        // DOS PATH length limit is 66 bytes.
        let mut filename_array: [u8; 70] = [0; 70]; // To be sure of the segment
        for i in 0..min(filename_array.len(), filename.len()) {
            filename_array[i] = filename.as_bytes()[i];
        }
        let filename_ptr = filename_array.as_ptr();
        unsafe {
            asm!("mov al, 0x40", "mov ah, 0x3d", "int 0x21", "setc  dl", "movzx cx, dl", in("dx") filename_ptr as u16, lateout("cx") is_open_success, lateout("ax") error_code_or_handle);
        }
        if is_open_success == 1 {
            return Err(());
        }
        Ok(Self {
            handle: error_code_or_handle,
        })
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, ()> {
        let mut total_bytes_read: usize = 0;
        for buffer_write_pos in 0..buffer.len() {
            let mut is_read_success: u16 = 1; // 0: success, 1: fail
            let mut error_code_or_bytes_read: u16 = 0;
            let mut tmp_stack_buffer: [u8; 1] = [0; 1]; // To be sure of the segment
            let tmp_buffer_ptr = tmp_stack_buffer.as_mut_ptr();
            unsafe {
                let mut registers: [u16; 4] = [0; 4]; // Save registers content on the stack
                asm!("nop", out("ax") registers[0], out("bx") registers[1], out("cx") registers[2], out("dx") registers[3]);
                asm!("mov cx, 1", "mov ah, 0x3f", "int 0x21", "setc  dl", "movzx cx, dl", in("bx") self.handle, in("dx") tmp_buffer_ptr, lateout("cx") is_read_success, lateout("ax") error_code_or_bytes_read);
                asm!("nop", in("ax") registers[0], in("bx") registers[1], in("cx") registers[2], in("dx") registers[3]);
            }
            if is_read_success == 1 {
                return Err(());
            }
            if error_code_or_bytes_read == 0 {
                // End of file
                break;
            }

            total_bytes_read += error_code_or_bytes_read as usize; // = 1
            buffer[buffer_write_pos] = tmp_stack_buffer[0];
        }

        // fill the rest of the buffer with 0s
        for buffer_write_pos in total_bytes_read..buffer.len() {
            buffer[buffer_write_pos] = 0;
        }
        Ok(total_bytes_read)
    }

    pub fn close(self) -> Result<(), ()> {
        let mut is_close_success: u16 = 1; // 0: success, 1: fail
        let mut _error_code: u16 = 0; // 6 = unknown handle
        unsafe {
            let mut registers: [u16; 4] = [0; 4]; // Save registers content on the stack
            asm!("nop", out("ax") registers[0], out("bx") registers[1], out("cx") registers[2], out("dx") registers[3]);
            asm!("mov ah, 0x3e", "int 0x21", "setc  dl", "movzx cx, dl", in("bx") self.handle, lateout("cx") is_close_success, lateout("ax") _error_code);
            asm!("nop", in("ax") registers[0], in("bx") registers[1], in("cx") registers[2], in("dx") registers[3]);
        }
        if is_close_success == 1 {
            return Err(());
        }
        Ok(())
    }
}
