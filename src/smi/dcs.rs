#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Direct control register
#define SMI_DCS_FIELDS \
    enable:1, start:1, done:1, write:1
REG_DEF(SMI_DCS_REG, SMI_DCS_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_DCS;

pub struct DCS {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_dcs_base: *mut u32,
}

impl DCS {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_DCS + 4);

        // init dcs fields to 0
        let smi_dcs_base = unsafe {
            let smi_dcs_base = mapping.as_ptr().byte_add(SMI_DCS) as *mut u32;
            *smi_dcs_base = 0;
            smi_dcs_base
        };

        drop(mapping);
        DCS {
            smi_map,
            smi_dcs_base,
        }
    }

    pub fn get_enable(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_dcs_base) };
        (reg & 1) != 0
    }

    pub fn set_enable(&self, enable: bool) {
        let mut reg = unsafe { read_volatile(self.smi_dcs_base) };
        if enable {
            reg |= 1;
        } else {
            reg &= !1;
        }
        unsafe { write_volatile(self.smi_dcs_base, reg); }
    }

    pub fn get_start(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_dcs_base) };
        (reg & 2) != 0
    }

    pub fn set_start(&self, start: bool) {
        let mut reg = unsafe { read_volatile(self.smi_dcs_base) };
        if start {
            reg |= 2;
        } else {
            reg &= !2;
        }
        unsafe { write_volatile(self.smi_dcs_base, reg); }
    }

    pub fn get_done(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_dcs_base) };
        (reg & 4) != 0
    }

    pub fn set_done(&self, done: bool) {
        let mut reg = unsafe { read_volatile(self.smi_dcs_base) };
        if done {
            reg |= 4;
        } else {
            reg &= !4;
        }
        unsafe { write_volatile(self.smi_dcs_base, reg); }
    }

    pub fn get_write(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_dcs_base) };
        (reg & 8) != 0
    }

    pub fn set_write(&self, write: bool) {
        let mut reg = unsafe { read_volatile(self.smi_dcs_base) };
        if write {
            reg |= 8;
        } else {
            reg &= !8;
        }
        unsafe { write_volatile(self.smi_dcs_base, reg); }
    }
}