#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Debug register
#define SMI_FD_FIELDS \
    fcnt:6, _x1:2, flvl:6
REG_DEF(SMI_FD_REG, SMI_FD_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_FD;

pub struct FD {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_fd_base: *mut u32,
}

impl FD {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_FD + 4);

        // init fd fields to 0
        let smi_fd_base = unsafe {
            let smi_fd_base = mapping.as_ptr().byte_add(SMI_FD) as *mut u32;
            *smi_fd_base = 0;
            smi_fd_base
        };

        drop(mapping);
        FD {
            smi_map,
            smi_fd_base,
        }
    }

    pub fn get_fcnt(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_fd_base) };
        (reg & 0x3F) as u8
    }

    pub fn set_fcnt(&self, fcnt: u8) {
        let mut reg = unsafe { read_volatile(self.smi_fd_base) };
        reg = (reg & !0x3F) | (fcnt as u32 & 0x3F);
        unsafe { write_volatile(self.smi_fd_base, reg); }
    }

    pub fn get_flvl(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_fd_base) };
        ((reg >> 8) & 0x3F) as u8
    }

    pub fn set_flvl(&self, flvl: u32) {
        let mut reg = unsafe { read_volatile(self.smi_fd_base) };
        reg = (reg & !0x3F00) | ((flvl as u32 & 0x3F) << 8);
        unsafe { write_volatile(self.smi_fd_base, reg); }
    }
}