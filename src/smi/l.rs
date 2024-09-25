#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Data length register
#define SMI_L_FIELDS \
    len:32
REG_DEF(SMI_L_REG, SMI_L_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_L;

pub struct L {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_l_base: *mut u32,
}

impl L {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_L + 4);

        // init l fields to 0
        let smi_l_base = unsafe {
            let smi_l_base = mapping.as_ptr().byte_add(SMI_L) as *mut u32;
            *smi_l_base = 0;
            smi_l_base
        };

        drop(mapping);
        L {
            smi_map,
            smi_l_base,
        }
    }

    pub fn get_len(&self) -> u32 {
        let reg = unsafe { read_volatile(self.smi_l_base) };
        reg
    }

    pub fn set_len(&self, len: u32) {
        unsafe { write_volatile(self.smi_l_base, len); }
    }
}
