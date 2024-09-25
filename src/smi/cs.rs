#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use std::ptr::{read_volatile, write_volatile};

use log::debug;
use memmap2::MmapMut;

use crate::{r, rwm};
use crate::smi::SMI_CS;

pub struct CS {
    smi_map: Rc<RefCell<MmapMut>>,
    smi_cs_base: *mut u32,
}

impl CS {
    pub fn new(smi_map: Rc<RefCell<MmapMut>>) -> Self {
        let smi_map = smi_map.clone();
        let mapping = smi_map.borrow();
        assert!(mapping.len() >= SMI_CS + 4);

        // init cs fields to 0
        let smi_cs_base = unsafe {
            let smi_cs_base = mapping.as_ptr().byte_add(SMI_CS) as *mut u32;
            *smi_cs_base = 0;
            smi_cs_base
        };

        drop(mapping);
        CS {
            smi_map,
            smi_cs_base,
        }
    }

    pub fn get_enable(&self) -> bool {
        r(self.smi_cs_base) & 1 != 0
    }

    pub fn set_enable(&self, enable: bool) {
        rwm(self.smi_cs_base, |reg| {
            if enable {
                *reg |= 1
            } else {
                *reg &= !1
            }
        });
    }
    
    pub fn get_done(&self) -> bool {
        r(self.smi_cs_base) & (1 << 1) != 0
    }

    pub fn set_done(&self, done: bool) {
        rwm(self.smi_cs_base, |reg| {
            if done {
                *reg |= 1 << 1
            } else {
                *reg &= !(1 << 1)
            }
        });
    }

    pub fn get_active(&self) -> bool {
        r(self.smi_cs_base) & (1 << 2) != 0
    }

    pub fn set_start(&self, start: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if start {
            reg |= 1 << 3;
        } else {
            reg &= !(1 << 3);
        }
        debug!("set_start: {:32b}", reg);
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn set_clear(&self, clear: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if clear {
            reg |= 1 << 4;
        } else {
            reg &= !(1 << 4);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_write(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 5) != 0
    }

    pub fn set_write(&self, write: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if write {
            reg |= 1 << 5;
        } else {
            reg &= !(1 << 5);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_teen(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 8) != 0
    }

    pub fn set_teen(&self, teen: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if teen {
            reg |= 1 << 8;
        } else {
            reg &= !(1 << 8);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_intd(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 9) != 0
    }

    pub fn set_intd(&self, intd: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if intd {
            reg |= 1 << 9;
        } else {
            reg &= !(1 << 9);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_intt(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 10) != 0
    }

    pub fn set_intt(&self, intt: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if intt {
            reg |= 1 << 10;
        } else {
            reg &= !(1 << 10);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_intr(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 11) != 0
    }

    pub fn set_intr(&self, intr: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if intr {
            reg |= 1 << 11;
        } else {
            reg &= !(1 << 11);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_pvmode(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 12) != 0
    }

    pub fn set_pvmode(&self, pvmode: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if pvmode {
            reg |= 1 << 12;
        } else {
            reg &= !(1 << 12);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_seterr(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 13) != 0
    }

    pub fn set_seterr(&self, seterr: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if seterr {
            reg |= 1 << 13;
        } else {
            reg &= !(1 << 13);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_pxldat(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 14) != 0
    }

    pub fn set_pxldat(&self, pxldat: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if pxldat {
            reg |= 1 << 14;
        } else {
            reg &= !(1 << 14);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_edreq(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 15) != 0
    }

    pub fn get_aferr(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 25) != 0
    }

    pub fn set_aferr(&self, aferr: bool) {
        let mut reg = unsafe { read_volatile(self.smi_cs_base) };
        if aferr {
            reg |= 1 << 25;
        } else {
            reg &= !(1 << 25);
        }
        unsafe { write_volatile(self.smi_cs_base, reg); }
    }

    pub fn get_txw(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 26) != 0
    }

    pub fn get_rxr(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 27) != 0
    }

    pub fn get_txd(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 28) != 0
    }

    pub fn get_rxd(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 29) != 0
    }

    pub fn get_txe(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 30) != 0
    }

    pub fn get_rxf(&self) -> bool {
        let reg = unsafe { read_volatile(self.smi_cs_base) };
        reg & (1 << 31) != 0
    }

    pub fn get_value(&self) -> u32 {
        unsafe { read_volatile(self.smi_cs_base) }
    }

    pub fn set_value(&self, value: u32) {
        unsafe { write_volatile(self.smi_cs_base, value); }
    }
}