#![allow(dead_code)]
/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Address & device number
#define SMI_A_FIELDS \
    addr:6, _x1:2, dev:2
REG_DEF(SMI_A_REG, SMI_A_FIELDS);
*/

use std::cell::RefCell;
use std::rc::Rc;

use memmap2::MmapMut;

use crate::{r, rwm};
use crate::smi::SMI_A;

pub struct A {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_a_base: *mut u32,
}

impl A {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_A + 4);

        // init a fields to 0
        let smi_a_base = unsafe {
            let smi_a_base = mapping.as_ptr().byte_add(SMI_A) as *mut u32;
            *smi_a_base = 0;
            smi_a_base
        };

        drop(mapping);
        A {
            smi_map,
            smi_a_base,
        }
    }

    pub fn get_addr(&self) -> u8 {
        (r(self.smi_a_base) & 0x3F) as u8
    }

    pub fn set_addr(&self, addr: u8) {
        rwm(self.smi_a_base, |reg| {
            *reg = (*reg & !0x3F) | (addr as u32 & 0x3F);
        });
    }

    pub fn get_dev(&self) -> u8 {
        ((r(self.smi_a_base) >> 8) & 0x3) as u8
    }

    pub fn set_dev(&self, dev: u8) {
        rwm(self.smi_a_base, |reg| {
            *reg = (*reg & !(0x3 << 8)) | ((dev as u32 & 0x3) << 8);
        });
    }
}
