#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Device settings: read (1 of 4)
#define SMI_DSR_FIELDS \
    rstrobe:7, rdreq:1, rpace:7, rpaceall:1, \
    rhold:6, fsetup:1, mode68:1, rsetup:6, rwidth:2
REG_DEF(SMI_DSR_REG, SMI_DSR_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_DSR;

pub struct DSR {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_dsr_base: *mut u32,
}

impl DSR {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>, channel: usize) -> Self {
        assert!(channel < 4);

        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_DSR + (8 * channel) + 4);

        // init dsr fields to 0
        let smi_dsr_base = unsafe {
            let smi_dsr_base = mapping.as_ptr().byte_add(SMI_DSR + (8 * channel)) as *mut u32;
            *smi_dsr_base = 0;
            smi_dsr_base
        };

        drop(mapping);
        DSR {
            smi_map,
            smi_dsr_base,
        }
    }

    fn write(&self, value: u32) {
        unsafe {
            write_volatile(self.smi_dsr_base, value);
        }
    }

    fn read(&self) -> u32 {
        unsafe { read_volatile(self.smi_dsr_base) }
    }

    pub fn get_rstrobe(&self) -> u8 {
        let reg = self.read();
        ((reg >> 0) & 0x7F) as u8
    }

    pub fn set_rstrobe(&self, value: u8) {
        let mut reg = self.read();
        reg = (reg & !(0x7F << 0)) | ((value as u32 & 0x7F) << 0);
        self.write(reg);
    }

    pub fn get_rdreq(&self) -> bool {
        let reg = self.read();
        (reg >> 7) & 0x1 != 0
    }

    pub fn set_rdreq(&self, value: bool) {
        let mut reg = self.read();
        reg = (reg & !(0x1 << 7)) | ((value as u32 & 0x1) << 7);
        self.write(reg);
    }

    pub fn get_rpace(&self) -> u8 {
        let reg = self.read();
        ((reg >> 8) & 0x7F) as u8
    }

    pub fn set_rpace(&self, value: u8) {
        let mut reg = self.read();
        reg = (reg & !(0x7F << 8)) | ((value as u32 & 0x7F) << 8);
        self.write(reg);
    }

    pub fn get_rpaceall(&self) -> bool {
        let reg = self.read();
        (reg >> 15) & 0x1 != 0
    }

    pub fn set_rpaceall(&self, value: bool) {
        let mut reg = self.read();
        reg = (reg & !(0x1 << 15)) | ((value as u32 & 0x1) << 15);
        self.write(reg);
    }

    pub fn get_rhold(&self) -> u8 {
        let reg = self.read();
       ((reg >> 16) & 0x3F) as u8
    }

    pub fn set_rhold(&self, value: u8) {
        let mut reg = self.read();
        reg = (reg & !(0x3F << 16)) | ((value as u32 & 0x3F) << 16);
        self.write(reg);
    }

    pub fn get_fsetup(&self) -> bool {
        let reg = self.read();
        (reg >> 22) & 0x1 != 0
    }

    pub fn set_fsetup(&self, value: bool) {
        let mut reg = self.read();
        reg = (reg & !(0x1 << 22)) | ((value as u32 & 0x1) << 22);
        self.write(reg);
    }

    pub fn get_mode68(&self) -> bool {
        let reg = self.read();
        (reg >> 23) & 0x1 != 0
    }

    pub fn set_mode68(&self, value: bool) {
        let mut reg = self.read();
        reg = (reg & !(0x1 << 23)) | ((value as u32 & 0x1) << 23);
        self.write(reg);
    }

    pub fn get_rsetup(&self) -> u8 {
        let reg = self.read();
        ((reg >> 24) & 0x3F) as u8
    }

    pub fn set_rsetup(&self, value: u8) {
        let mut reg = self.read();
        reg = (reg & !(0x3F << 24)) | ((value as u32 & 0x3F) << 24);
        self.write(reg);
    }

    pub fn get_rwidth(&self) -> u8 {
        let reg = self.read();
        ((reg >> 30) & 0x3) as u8
    }

    pub fn set_rwidth(&self, value: u8) {
        let mut reg = self.read();
        reg = (reg & !(0x3 << 30)) | ((value as u32 & 0x3) << 30);
        self.write(reg);
    }
}