// Allocator for MS-DOS, using 21h 48h and 49h functions.
// The allocator is a simple linear allocator. It is used to allocate memory for the DOS kernel.

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;

pub struct DosAllocator;

#[allow(unused_assignments)]
#[allow(unused_variables)]
unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ret: *mut u8;
        let mut alloc_success: u16 = 1; // 0 = success, 1 = failure
        let num_pages: u16 = layout.size() as u16 / 16;
        asm!("mov ah, 0x48",
             "int 0x21",
             in("bx") num_pages,
             out("ax") ret);
        asm!("setc dl",
        "movzx edx, dl",
        out("edx") alloc_success);
        if alloc_success == 1 {
            panic!("DosAllocator: alloc failed");
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut dealloc_success: u16 = 1; // 0 = success, 1 = failure
        asm!("mov ah, 0x49",
             "mov dx, es",
             "int 0x21",
             in("dx") ptr);
        asm!("setc dl",
              "movzx edx, dl",
               out("edx") dealloc_success);
        if dealloc_success == 1 {
            panic!("DosAllocator: dealloc failed");
        }
    }
}

pub static DOS_ALLOCATOR: DosAllocator = DosAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[global_allocator]
static GLOBAL_ALLOCATOR: DosAllocator = DosAllocator;
