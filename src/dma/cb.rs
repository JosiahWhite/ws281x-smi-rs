use std::ops::DerefMut;

use log::debug;

use crate::vc_mem::VcMem;

pub struct DmaControlBlock {
    memory: VcMem,
}

impl DmaControlBlock {
    pub fn new() -> Self {
        let mut memory = VcMem::new(0x20, 0x20);
        memory.fill(0);

        DmaControlBlock {
            memory,
        }
    }

    pub fn busaddr(&self) -> usize {
        debug!("DmaControlBlock::busaddr() = {:x}", self.memory.busaddr());
        self.memory.busaddr()
    }

    pub fn set_transfer_info(&mut self, value: u32) {
        unsafe {
            self.memory.deref_mut()
                .as_mut_ptr()
                .byte_add(0)
                .cast::<u32>()
                .write_volatile(value);
        }
    }

    pub fn set_source_address(&mut self, value: u32) {
        unsafe {
            self.memory.deref_mut()
                .as_mut_ptr()
                .byte_add(4)
                .cast::<u32>()
                .write_volatile(value);
        }
    }

    pub fn set_destination_address(&mut self, value: u32) {
        unsafe {
            self.memory.deref_mut()
                .as_mut_ptr()
                .byte_add(8)
                .cast::<u32>()
                .write_volatile(value);
        }
    }

    pub fn set_transfer_length(&mut self, value: u32) {
        unsafe {
            self.memory.deref_mut()
                .as_mut_ptr()
                .byte_add(12)
                .cast::<u32>()
                .write_volatile(value);
        }
    }

    pub fn set_transfer_stride(&mut self, value: u32) {
        unsafe {
            self.memory.deref_mut()
                .as_mut_ptr()
                .byte_add(16)
                .cast::<u32>()
                .write_volatile(value);
        }
    }

    pub fn set_next_control_block(&mut self, value: u32) {
        unsafe {
            self.memory.deref_mut()
                .as_mut_ptr()
                .byte_add(20)
                .cast::<u32>()
                .write_volatile(value);
        }
    }
}