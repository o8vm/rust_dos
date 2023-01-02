use alloc::collections::VecDeque;
use core::arch::{asm, global_asm};
use crate::dos::cooperative_multitasking::task::{Registers, Task};

mod task;

global_asm!(include_str!("cooperative_task_switching.S"));

extern "C" {
    fn cooperative_task_switching_assembly(from: *mut Registers, to: *mut Registers) -> ();
}

pub struct Tasking{
    task_list: Option<VecDeque<Task>>,
    current_task_id: u8,
    eflags_register: u32,
    cr3_register: u32,
    initialized: bool,
}

impl Tasking {
    const MAX_TASKS: usize = 10;

    pub fn init(&mut self) {
        if self.task_list.is_some() {
            self.task_list = None;
        }
        let (eflags, cr3) = Self::get_eflags_and_cr3_registers();

        // Create main task
        self.task_list = Some(VecDeque::with_capacity(Self::MAX_TASKS));
        self.current_task_id = 0;
        self.eflags_register = eflags;
        self.cr3_register = cr3;
        self.initialized = true;
        self.task_list.as_mut().unwrap().push_back(Task {
            registers: Registers {
                eax: 0,
                ebx: 0,
                ecx: 0,
                edx: 0,
                esi: 0,
                edi: 0,
                esp: 0,
                ebp: 0,
                eip: 0,
                eflags,
                cr3,
            },
        });
    }

    pub fn add_task(&mut self, main_function: *mut fn()) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Cooperative tasking manager is not initialized");
        }
        if self.task_list.as_ref().unwrap().len() >= Self::MAX_TASKS {
            return Err("Maximum number of tasks reached");
        }
        let task_list = self.task_list.as_mut().unwrap();
        task_list.push_back(Task::new(main_function, self.eflags_register, self.cr3_register as *mut u32, task_list.len() as u8));
        Ok(())
    }

    pub fn yield_task(&mut self) {
        if !self.initialized {
            panic!("Cooperative tasking manager is not initialized");
        }

        let task_list = self.task_list.as_mut().unwrap();

        let current_task_registers_ptr = &mut task_list[self.current_task_id as usize].registers as *mut Registers;

        self.current_task_id += 1;
        if self.current_task_id >= task_list.len() as u8 {
            self.current_task_id = 0;
        }

        let next_task_registers_ptr = &mut task_list[self.current_task_id as usize].registers as *mut Registers;

        unsafe {
            cooperative_task_switching_assembly(current_task_registers_ptr, next_task_registers_ptr);
        }
    }

    fn get_eflags_and_cr3_registers() -> (u32, u32) {
        let mut eflags: u32;
        let mut cr3: u32;
        unsafe {
            // Read CR3
            asm!("mov {}, cr3", out(reg) cr3);
            // Read EFLAGS
            asm!("pushfd; mov eax, [esp]; mov {}, eax; popfd;", out(reg) eflags);
        }
        (eflags, cr3)
    }
}

pub static mut TASKING: Tasking = Tasking {
    task_list: None,
    current_task_id: 0,
    eflags_register: 0,
    cr3_register: 0,
    initialized: false,
};

#[macro_export]
macro_rules! yield_cooperative_task {
    () => {
        unsafe {
            $crate::dos::cooperative_multitasking::TASKING.yield_task();
        }
    };
}

#[macro_export]
macro_rules! add_cooperative_task {
    ($main_function: expr) => {
        unsafe {
            $crate::dos::cooperative_multitasking::TASKING.add_task($main_function as *mut fn())
        }
    };
}