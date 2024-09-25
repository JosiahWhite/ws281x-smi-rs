#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Direct control data
#define SMI_DCD_FIELDS \
    data:32
REG_DEF(SMI_DCD_REG, SMI_DCD_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_DCD;

pub struct DCD {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_dcd_base: *mut u32,
}

impl DCD {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_DCD + 4);

        // init dcd fields to 0
        let smi_dcd_base = unsafe {
            let smi_dcd_base = mapping.as_ptr().byte_add(SMI_DCD) as *mut u32;
            smi_dcd_base
        };

        drop(mapping);
        DCD {
            smi_map,
            smi_dcd_base,
        }
    }

    pub fn get_data(&self) -> u32 {
        let reg = unsafe { read_volatile(self.smi_dcd_base) };
        reg
    }

    pub fn set_data(&self, data: u32) {
        unsafe { write_volatile(self.smi_dcd_base, data); }
    }
}