#[repr(C)] // To ensure that the struct is laid out in the same way as the assembly code expects
#[derive(Copy, Clone, Debug)]
pub(crate)struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub esp: u32,
    pub ebp: u32,
    pub eip: u32,
    pub eflags: u32,
    pub cr3: u32,
}

#[derive(Debug)]
pub(crate) struct Task {
    pub(crate) registers: Registers,
}

// In order to use heap as stack, we need to change ss stack segment register
impl Task {
    const TASK_STACK_SIZE: usize = 4096;

    /// Max stack for each task, including the main task, is 4KB
    pub fn new(main_function: *mut fn(), flags: u32, pagedir: *mut u32, task_index: u8) -> Task {
        Task {
            registers: Registers {
                eax: 0,
                ebx: 0,
                ecx: 0,
                edx: 0,
                esi: 0,
                edi: 0,
                esp: 0xffff as u32 - (Self::TASK_STACK_SIZE as u32 * task_index as u32),
                ebp: 0,
                eip: main_function as u32,
                eflags: flags,
                cr3: pagedir as u32,
            }
        }
    }
}