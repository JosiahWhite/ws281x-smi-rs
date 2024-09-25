#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Direct control address & device number
#define SMI_DCA_FIELDS \
    addr:6, _x1:2, dev:2
REG_DEF(SMI_DCA_REG, SMI_DCA_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_DCA;

pub struct DCA {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_dca_base: *mut u32,
}

impl DCA {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_DCA + 4);

        // init dca fields to 0
        let smi_dca_base = unsafe {
            let smi_dca_base = mapping.as_ptr().byte_add(SMI_DCA) as *mut u32;
            *smi_dca_base = 0;
            smi_dca_base
        };

        drop(mapping);
        DCA {
            smi_map,
            smi_dca_base,
        }
    }

    pub fn get_addr(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_dca_base) };
        (reg & 0x3F) as u8
    }

    pub fn set_addr(&self, addr: u8) {
        let mut reg = unsafe { read_volatile(self.smi_dca_base) };
        reg = (reg & !0x3F) | (addr as u32 & 0x3F);
        unsafe { write_volatile(self.smi_dca_base, reg); }
    }

    pub fn get_dev(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_dca_base) };
        ((reg >> 8) & 0x3) as u8
    }

    pub fn set_dev(&self, dev: u8) {
        let mut reg = unsafe { read_volatile(self.smi_dca_base) };
        reg = (reg & !(0x3 << 8)) | ((dev as u32 & 0x3) << 8);
        unsafe { write_volatile(self.smi_dca_base, reg); }
    }
}