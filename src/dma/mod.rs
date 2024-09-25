mod cb;

use std::fs::OpenOptions;

pub use cb::DmaControlBlock;
use log::debug;
use memmap2::{MmapMut, MmapOptions};
use once_cell::sync::OnceCell;

use crate::{r, rwm, w, DMA_BASE_ADDRESS};

const DMA_CS: usize        = 0x00;
const DMA_CONBLK_AD: usize = 0x04;
const DMA_TI: usize        = 0x08;
const DMA_SRCE_AD: usize   = 0x0c;
const DMA_DEST_AD: usize   = 0x10;
const DMA_TXFR_LEN: usize  = 0x14;
const DMA_STRIDE: usize    = 0x18;
const DMA_NEXTCONBK: usize = 0x1c;
const DMA_DEBUG: usize     = 0x20;
const DMA_ENABLE: usize = 0xff0;

// DMA register values
pub const DMA_WAIT_RESP: usize   = 1 << 3;
pub const DMA_CB_DEST_INC: usize = 1 << 4;
pub const DMA_DEST_DREQ: usize   = 1 << 6;
pub const DMA_CB_SRCE_INC: usize = 1 << 8;
pub const DMA_SRCE_DREQ: usize   = 1 << 10;
pub const fn dma_priority(n: usize) -> usize { n << 16 }


pub struct Dma {
    channel: u8,
    channel_base: *mut u32,
    dma_map: MmapMut,
    control_block: DmaControlBlock,
    reset: OnceCell<()>,
}

impl Dma {
    pub fn new(channel: u8) -> Self {
        let devmem = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")
            .expect("Failed to open /dev/mem");

        assert!(channel < 15);

        let mut dma_map = unsafe {
            MmapOptions::new()
                .offset(DMA_BASE_ADDRESS as u64)
                .len(0xa000)
                .map_mut(&devmem)
                .expect("Failed to map DMA memory")
        };

        let control_block = DmaControlBlock::new();

        let channel_base = unsafe {
            dma_map
                .as_mut_ptr()
                .byte_add(channel as usize * 0x100)
                .cast::<u32>()
        };

        Dma {
            channel,
            channel_base,
            dma_map,
            control_block,
            reset: OnceCell::new(),
        }
    }

    pub fn get_cb(&mut self) -> &mut DmaControlBlock {
        &mut self.control_block
    }

    pub fn enable(&mut self) {
        let reg = unsafe {
            self.dma_map
                .as_mut_ptr()
                .byte_add(DMA_ENABLE)
                .cast::<u32>()
        };

        rwm(reg, |value| *value |= 1 << self.channel);

        // reset the dma peripheral once
        self.reset.get_or_init(|| {
            let reg = unsafe {
                self.channel_base.byte_add(DMA_CS)
            };

            rwm(reg, |value| *value |= 1 << 31);
        });
    }

    pub fn disable(&mut self) {
        let reg = unsafe {
            self.dma_map
                .as_mut_ptr()
                .byte_add(DMA_ENABLE)
                .cast::<u32>()
        };

        rwm(reg, |value| *value &= !(1 << self.channel));
    }

    pub fn reset(&mut self) {
        let reg = unsafe {
            self.channel_base.byte_add(DMA_CS)
        };

        rwm(reg, |value| *value |= 1 << 31);
    }

    pub fn start(&mut self) {
        unsafe {
            let cbad = self.channel_base.byte_add(DMA_CONBLK_AD);
            let addr = self.control_block.busaddr();
            w(cbad, addr as u32);

            let cs = self.channel_base.byte_add(DMA_CS);
            w(cs, 2); // clear end flag

            let debug = self.channel_base.byte_add(DMA_DEBUG);
            w(debug, 7); // clear error bits

            w(cs, 1); // start transfer
        };
    }

    pub fn wait(&self) {
        let addr = unsafe { self.channel_base.byte_add(DMA_CS) };
        while r(addr) & 1 != 0 {}
    }
}