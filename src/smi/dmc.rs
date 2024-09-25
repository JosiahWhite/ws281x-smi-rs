#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// DMA control register
#define SMI_DMC_FIELDS \
    reqw:6, reqr:6, panicw:6, panicr:6, dmap:1, _x1:3, dmaen:1
REG_DEF(SMI_DMC_REG, SMI_DMC_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_DMC;

pub struct DMC {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_dmc_base: *mut u32,
}

impl DMC {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_DMC + 4);

        // init dmc fields to 0
        let smi_dmc_base = unsafe {
            let smi_dmc_base = mapping.as_ptr().byte_add(SMI_DMC) as *mut u32;
            *smi_dmc_base = 0;
            smi_dmc_base
        };

        drop(mapping);
        DMC {
            smi_map,
            smi_dmc_base,
        }
    }

    pub fn get_reqw(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_dmc_base) };
        ((reg >> 0) & 0x3f) as u8
    }

    pub fn set_reqw(&self, reqw: u8) {
        let mut reg = unsafe { read_volatile(self.smi_dmc_base) };
        reg &= !(0x3f << 0);
        reg |= (reqw as u32 & 0x3f) << 0;
        unsafe { write_volatile(self.smi_dmc_base, reg); }
    }

    pub fn get_reqr(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_dmc_base) };
        ((reg >> 6) & 0x3f) as u8
    }

    pub fn set_reqr(&self, reqr: u8) {
        let mut reg = unsafe { read_volatile(self.smi_dmc_base) };
        reg &= !(0x3f << 6);
        reg |= (reqr as u32 & 0x3f) << 6;
        unsafe { write_volatile(self.smi_dmc_base, reg); }
    }

    pub fn get_panicw(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_dmc_base) };
        ((reg >> 12) & 0x3f) as u8
    }

    pub fn set_panicw(&self, panicw: u8) {
        let mut reg = unsafe { read_volatile(self.smi_dmc_base) };
        reg &= !(0x3f << 12);
        reg |= (panicw as u32 & 0x3f) << 12;
        unsafe { write_volatile(self.smi_dmc_base, reg); }
    }

    pub fn get_panicr(&self) -> u8 {
        let reg = unsafe { read_volatile(self.smi_dmc_base) };
        ((reg >> 18) & 0x3f) as u8
    }

    pub fn set_panicr(&self, panicr: u8) {
        let mut reg = unsafe { read_volatile(self.smi_dmc_base) };
        reg &= !(0x3f << 18);
        reg |= (panicr as u32 & 0x3f) << 18;
        unsafe { write_volatile(self.smi_dmc_base, reg); }
    }

    pub fn get_dmap(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_dmc_base) };
        (reg >> 24) & 0x1 != 0
    }

    pub fn set_dmap(&self, dmap: bool) {
        let mut reg = unsafe { read_volatile(self.smi_dmc_base) };
        reg &= !(0x1 << 24);
        reg |= (dmap as u32 & 0x1) << 24;
        unsafe { write_volatile(self.smi_dmc_base, reg); }
    }

    pub fn get_dmaen(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_dmc_base) };
        (reg >> 28) & 0x1 != 0
    }

    pub fn set_dmaen(&self, dmaen: bool) {
        let mut reg = unsafe { read_volatile(self.smi_dmc_base) };
        reg &= !(0x1 << 28);
        reg |= (dmaen as u32 & 0x1) << 28;
        unsafe { write_volatile(self.smi_dmc_base, reg); }
    }
}
