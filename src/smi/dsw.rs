#![allow(dead_code)]

/*
// Union of 32-bit value with register bitfields
#define REG_DEF(name, fields) typedef union {struct {volatile uint32_t fields;}; volatile uint32_t value;} name

// Device settings: write (1 of 4)
#define SMI_DSW_FIELDS \
    wstrobe:7, wdreq:1, wpace:7, wpaceall:1, \
    whold:6, wswap:1, wformat:1, wsetup:6, wwidth:2
REG_DEF(SMI_DSW_REG, SMI_DSW_FIELDS);
*/

use std::cell::RefCell;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use memmap2::MmapMut;

use crate::smi::SMI_DSW;

pub struct DSW {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_dsw_base: *mut u32,
}

impl DSW {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>, channel: usize) -> Self {
        assert!(channel < 4);

        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_DSW + (8 * channel) + 4);

        // init dsw fields to 0
        let smi_dsw_base = unsafe {
            let smi_dsw_base = mapping.as_ptr().byte_add(SMI_DSW + (8 * channel)) as *mut u32;
            *smi_dsw_base = 0;
            smi_dsw_base
        };

        drop(mapping);
        DSW {
            smi_map,
            smi_dsw_base,
        }
    }

    fn write(&self, value: u32) {
        unsafe {
            write_volatile(self.smi_dsw_base, value);
        }
    }

    fn read(&self) -> u32 {
        unsafe { read_volatile(self.smi_dsw_base) }
    }

    pub fn get_wstrobe(&self) -> u8 {
        ((self.read() >> 0) & 0x7F) as u8
    }

    pub fn set_wstrobe(&self, wstrobe: u8) {
        let mut value = self.read();
        value = (value & !(0x7F << 0)) | ((wstrobe as u32 & 0x7F) << 0);
        self.write(value);
    }

    pub fn get_wdreq(&self) -> bool {
        (self.read() >> 7) & 0x1 != 0
    }

    pub fn set_wdreq(&self, wdreq: bool) {
        let mut value = self.read();
        value = (value & !(0x1 << 7)) | ((wdreq as u32 & 0x1) << 7);
        self.write(value);
    }

    pub fn get_wpace(&self) -> u8 {
        ((self.read() >> 8) & 0x7F) as u8
    }

    pub fn set_wpace(&self, wpace: u8) {
        let mut value = self.read();
        value = (value & !(0x7F << 8)) | ((wpace as u32 & 0x7F) << 8);
        self.write(value);
    }

    pub fn get_wpaceall(&self) -> bool {
        (self.read() >> 15) & 0x1 != 0
    }

    pub fn set_wpaceall(&self, wpaceall: bool) {
        let mut value = self.read();
        value = (value & !(0x1 << 15)) | ((wpaceall as u32 & 0x1) << 15);
        self.write(value);
    }

    pub fn get_whold(&self) -> u8 {
        ((self.read() >> 16) & 0x3F) as u8
    }

    pub fn set_whold(&self, whold: u8) {
        let mut value = self.read();
        value = (value & !(0x3F << 16)) | ((whold as u32 & 0x3F) << 16);
        self.write(value);
    }

    pub fn get_wswap(&self) -> bool {
        (self.read() >> 22) & 0x1 != 0
    }

    pub fn set_wswap(&self, wswap: bool) {
        let mut value = self.read();
        value = (value & !(0x1 << 22)) | ((wswap as u32 & 0x1) << 22);
        self.write(value);
    }

    pub fn get_wformat(&self) -> bool {
        (self.read() >> 23) & 0x1 != 0
    }

    pub fn set_wformat(&self, wformat: bool) {
        let mut value = self.read();
        value = (value & !(0x1 << 23)) | ((wformat as u32 & 0x1) << 23);
        self.write(value);
    }

    pub fn get_wsetup(&self) -> u8 {
        ((self.read() >> 24) & 0x3F) as u8
    }

    pub fn set_wsetup(&self, wsetup: u8) {
        let mut value = self.read();
        value = (value & !(0x3F << 24)) | ((wsetup as u32 & 0x3F) << 24);
        self.write(value);
    }

    pub fn get_wwidth(&self) -> u8 {
        ((self.read() >> 30) & 0x3) as u8
    }

    pub fn set_wwidth(&self, wwidth: u8) {
        let mut value = self.read();
        value = (value & !(0x3 << 30)) | ((wwidth as u32 & 0x3) << 30);
        self.write(value);
    }
}
