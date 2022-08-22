// Allocator for MS-DOS, using 21h 48h and 49h functions.
// The allocator is a simple linear allocator. It is used to allocate memory for the DOS kernel.

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::cmp::min;
use core::mem::size_of;

struct AllocatorBlock {
    next: Option<*mut AllocatorBlock>,
    size: usize,
    used: bool,
}

pub struct DosAllocator {
    first_block_ptr: *mut AllocatorBlock,
}

// Todo: check total memory amount
#[allow(unused_assignments)]
#[allow(unused_variables)]
unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut current_block_ptr = self.first_block_ptr;
        while (*current_block_ptr).used || ((*current_block_ptr).size != 0 && (*current_block_ptr).size < layout.size() + size_of::<AllocatorBlock>()) {
            current_block_ptr = (*current_block_ptr).next.unwrap();
        }
        (*current_block_ptr).used = true;
        (*current_block_ptr).size = layout.size();
        let new_block_ptr = ((*current_block_ptr).size as u32 + current_block_ptr as u32 + size_of::<AllocatorBlock>() as u32) as *mut AllocatorBlock;
        (*new_block_ptr).next = None;
        (*new_block_ptr).used = false;
        (*new_block_ptr).size = 0;
        (*current_block_ptr).next = Some(new_block_ptr);
        let ret_ptr = current_block_ptr as u32 + size_of::<AllocatorBlock>() as u32;
        ret_ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let current_block_ptr = (ptr as u32 - size_of::<AllocatorBlock>() as u32) as *mut AllocatorBlock;
        (*current_block_ptr).used = false;
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let current_block_ptr = (ptr as u32 - size_of::<AllocatorBlock>() as u32) as *mut AllocatorBlock;
        if (*current_block_ptr).size >= new_size {
            return ptr;
        }
        let new_ptr = self.alloc(Layout::from_size_align(new_size, layout.align()).unwrap());
        new_ptr.copy_from_nonoverlapping(ptr, min(layout.size(), new_size));
        self.dealloc(ptr, layout);
        new_ptr
    }
}

impl DosAllocator {
    pub fn init(&mut self) {
        let mut _heap_segment_addr: u32 = 0;
        unsafe {
            asm!("mov ax, _heap", out("ax") _heap_segment_addr);
        }
        let mut _heap_ptr_as_block = ((_heap_segment_addr & 0xFFFF) << 4) as *mut AllocatorBlock;
        unsafe {
            *_heap_ptr_as_block = AllocatorBlock {
                next: None,
                size: 0,
                used: false,
            };
        }
        self.first_block_ptr = _heap_ptr_as_block
    }

    const fn new() -> Self {
        Self {
            first_block_ptr: core::ptr::null_mut(),
        }
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[global_allocator]
pub(crate) static mut GLOBAL_ALLOCATOR: DosAllocator = DosAllocator::new();