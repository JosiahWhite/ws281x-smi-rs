#![allow(dead_code)]

use std::fs::OpenOptions;
use std::ops::{Deref, DerefMut};

use memmap2::{MmapMut, MmapOptions};
use rpi_mailbox::{
    mailbox_mem_alloc,
    mailbox_mem_free,
    mailbox_mem_lock,
    mailbox_mem_unlock,
    memflag,
    Mailbox
};

pub struct VcMem {
    mb: Mailbox,
    handle: u32,
    busaddr: usize,
    physaddr: usize,
    mapping: Option<MmapMut>,
}

impl VcMem {
    pub fn new(size: u32, alignment: u32) -> Self {
        let mb = Mailbox::new("/dev/vcio").expect("mailbox");

        let devmem = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")
            .expect("Failed to open /dev/mem");

        let handle = mailbox_mem_alloc(
            &mb,
            size,
            alignment,
            memflag::Flags::MEM_FLAG_DIRECT | memflag::Flags::MEM_FLAG_ZERO
        ).expect("Failed to allocate memory");
        let busaddr = mailbox_mem_lock(&mb, handle).map_err(|err| {
            mailbox_mem_free(&mb, handle).ok();
            err
        }).expect("Failed to lock memory");

        let busaddr = busaddr as usize;
        let physaddr = (busaddr & !0xC0000000) as usize;

        let mapping = unsafe {
            MmapOptions::new()
                .offset(physaddr as u64)
                .len(size as usize)
                .map_mut(&devmem)
                .expect("Failed to map memory")
        };

        Self {
            mb,
            handle,
            busaddr,
            physaddr,
            mapping: Some(mapping),
        }
    }

    #[inline]
    pub fn as_ref(&self) -> &[u8] {
        self.mapping.as_ref().unwrap()
    }

    #[inline]
    pub fn as_mut(&mut self) -> &mut [u8] {
        self.mapping.as_mut().unwrap()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.mapping.as_ref().unwrap().as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.mapping.as_mut().unwrap().as_mut_ptr()
    }

    pub fn flush(&self) {
        self.mapping.as_ref().unwrap().flush().expect("Failed to flush memory");
    }

    pub fn busaddr(&self) -> usize {
        self.busaddr
    }
}

impl AsRef<[u8]> for VcMem {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsMut<[u8]> for VcMem {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

impl Deref for VcMem {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for VcMem {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl Drop for VcMem {
    fn drop(&mut self) {
        self.mapping.take();
        mailbox_mem_unlock(&self.mb, self.busaddr as u32).ok();
        mailbox_mem_free(&self.mb, self.handle).ok();
    }
}