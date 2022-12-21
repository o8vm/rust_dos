//! Memory heap allocation for DOS programs.
//! Uses conventional memory for DOS programs, from _heap segment start to extended BIOS data area (EBDA)
//! Uses linear algorithm for allocating memory, which is not optimal, but it's simple and works.

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::cmp::min;
use core::mem::size_of;

struct AllocatorBlock {
    next: Option<*mut AllocatorBlock>,
    prev: Option<*mut AllocatorBlock>,
    size: usize, // includes size of this block
    used: bool,
}

pub struct DosAllocator {
    first_block_ptr: *mut AllocatorBlock,
}

impl DosAllocator {
    const LAST_MEMORY_BYTE_ADDR: u32 = 0x9FBFF; // (0X9000 << 4) + 0XFBFF, last byte of memory before extended BIOS data area
    const ALLOCATOR_BLOCK_SIZE: usize = size_of::<AllocatorBlock>();
    const MIN_BLOCK_USEFUL_SIZE: usize = 16;

    fn diff_between_blocks_ptr(first_block: *mut AllocatorBlock, second_block: *mut AllocatorBlock) -> usize {
        let first_block_addr = first_block as usize;
        let second_block_addr = second_block as usize;
        assert!(first_block_addr < second_block_addr);
        second_block_addr - first_block_addr
    }

    fn free_space_before_next_block(block: *mut AllocatorBlock) -> usize {
        assert_ne!(block, core::ptr::null_mut());
        assert!((block as u32) < Self::LAST_MEMORY_BYTE_ADDR);
        let next_block = unsafe { (*block).next };
        if next_block.is_none() {
            return Self::LAST_MEMORY_BYTE_ADDR as usize - block as usize;
        }
        let next_block = next_block.unwrap();
        Self::diff_between_blocks_ptr(block, next_block) - Self::ALLOCATOR_BLOCK_SIZE
    }

    /// Converts block address to pointer usable by the program
    fn block_addr_to_useful_ptr(block: *mut AllocatorBlock) -> *mut u8 {
        assert_ne!(block, core::ptr::null_mut());
        (block as usize + Self::ALLOCATOR_BLOCK_SIZE) as *mut u8
    }
}

unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Look for next free block
        let mut current_block_ptr = self.first_block_ptr;
        while (*current_block_ptr).used || (*current_block_ptr).size < layout.size() + Self::ALLOCATOR_BLOCK_SIZE {
            current_block_ptr = match (*current_block_ptr).next {
                Some(ptr) => ptr,
                None => return core::ptr::null_mut(), // No free block found, return null ptr
            };
        }

        let free_space_before_next_block = Self::free_space_before_next_block(current_block_ptr);
        if free_space_before_next_block <= Self::MIN_BLOCK_USEFUL_SIZE + Self::ALLOCATOR_BLOCK_SIZE {
            // No space for new block, just use the whole space
            (*current_block_ptr).used = true;
            (*current_block_ptr).size = free_space_before_next_block;
            return Self::block_addr_to_useful_ptr(current_block_ptr);
        }

        // Create a new unused block between current and next

        let current_block_size = layout.size() + Self::ALLOCATOR_BLOCK_SIZE;

        // Create a new block just after current block
        let new_block_ptr = (current_block_size + current_block_ptr as usize) as *mut AllocatorBlock;
        (*new_block_ptr).next = (*current_block_ptr).next;
        (*new_block_ptr).prev = Some(current_block_ptr);
        (*new_block_ptr).size = free_space_before_next_block - current_block_size;
        (*new_block_ptr).used = false;

        (*current_block_ptr).next = Some(new_block_ptr);
        (*current_block_ptr).size = current_block_size;
        (*current_block_ptr).used = true;

        Self::block_addr_to_useful_ptr(current_block_ptr)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // Freeing null pointer is a no-op
        if ptr == core::ptr::null_mut() {
            return;
        }

        let current_block_ptr = (ptr as u32 - Self::ALLOCATOR_BLOCK_SIZE as u32) as *mut AllocatorBlock;
        // Mark block as free
        (*current_block_ptr).used = false;

        // Merge with next block if it's free
        let next_block_ptr = (*current_block_ptr).next;
        if next_block_ptr.is_some() {
            let next_block_ptr = next_block_ptr.unwrap();
            if !(*next_block_ptr).used {
                if (*next_block_ptr).next.is_some() {
                    (*(*next_block_ptr).next.unwrap()).prev = Some(current_block_ptr);
                }
                (*current_block_ptr).size += (*next_block_ptr).size;
                (*current_block_ptr).next = (*next_block_ptr).next;
            }
        }

        // Merge with previous block if it's free
        let prev_block_ptr = (*current_block_ptr).prev;
        if prev_block_ptr.is_some() {
            let prev_block_ptr = prev_block_ptr.unwrap();
            if !(*prev_block_ptr).used {
                if (*current_block_ptr).next.is_some() {
                    (*(*current_block_ptr).next.unwrap()).prev = Some(prev_block_ptr);
                }
                (*prev_block_ptr).size += (*current_block_ptr).size;
                (*prev_block_ptr).next = (*current_block_ptr).next;
            }
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        assert_ne!(ptr, core::ptr::null_mut()); // Avoid undefined behavior
        let current_block_ptr = (ptr as u32 - Self::ALLOCATOR_BLOCK_SIZE as u32) as *mut AllocatorBlock;
        if (*current_block_ptr).size >= new_size + Self::ALLOCATOR_BLOCK_SIZE {
            return ptr;
        }
        let new_ptr = self.alloc(Layout::from_size_align(new_size, layout.align()).unwrap());
        new_ptr.copy_from_nonoverlapping(ptr, min(layout.size(), new_size));
        self.dealloc(ptr, layout);
        new_ptr
    }
}

impl DosAllocator {
    #[allow(unused_assignments)]
    pub fn init(&mut self) {
        let mut heap_segment_number: u32 = 0;
        unsafe {
            asm!("mov ax, _heap", out("ax") heap_segment_number);
        }
        // Compute heap address from segment number
        let heap_addr = ((heap_segment_number & 0xFFFF) << 4) as u32;
        let heap_ptr_as_block = heap_addr as *mut AllocatorBlock;

        // Create an empty block at the beginning of the heap, containing all free space
        assert!(heap_addr as u32 <= DosAllocator::LAST_MEMORY_BYTE_ADDR);
        let first_block_size = DosAllocator::LAST_MEMORY_BYTE_ADDR - heap_addr;
        unsafe {
            *heap_ptr_as_block = AllocatorBlock {
                next: None,
                prev: None,
                size: first_block_size as usize,
                used: false,
            };
        }
        self.first_block_ptr = heap_ptr_as_block
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