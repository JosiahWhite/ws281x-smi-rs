#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Data FIFO
#define SMI_D_FIELDS \
    data:32
REG_DEF(SMI_D_REG, SMI_D_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_D;

pub struct D {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_d_base: *mut u32,
}

impl D {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= 4);

        // init d fields to 0
        let smi_d_base = unsafe {
            let smi_d_base = mapping.as_ptr().byte_add(SMI_D) as *mut u32;
            *smi_d_base = 0;
            smi_d_base
        };

        drop(mapping);
        D {
            smi_map,
            smi_d_base,
        }
    }

    pub fn get_data(&self) -> u32 {
        let reg = unsafe { read_volatile(self.smi_d_base) };
        reg
    }

    pub fn set_data(&self, data: u32) {
        unsafe { write_volatile(self.smi_d_base, data); }
    }
}