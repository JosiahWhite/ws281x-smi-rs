mod cs;
mod l;
mod a;
mod d;
mod dsr;
mod dsw;
mod dmc;
mod dcs;
mod dca;
mod dcd;
mod fd;

use std::cell::RefCell;
use std::fs::OpenOptions;
use std::ops::Range;
use std::rc::Rc;

use log::debug;
use memmap2::{MmapMut, MmapOptions};

use crate::{r, rwm, w};
use crate::dma::{Dma, DMA_CB_SRCE_INC, DMA_DEST_DREQ, DMA_WAIT_RESP};
use crate::vc_mem::VcMem;
use crate::{
    CLK_BASE_ADDRESS,
    PERIPHERAL_BASE_ADDRESS,
    PERIPHERAL_BUS_ADDRESS,
    REQUEST_THRESH,
    SMI_BASE_ADDRESS
};

const SMI_CS: usize   = 0x00;    // Control & status
const SMI_L: usize    = 0x04;    // Transfer length
const SMI_A: usize    = 0x08;    // Address
const SMI_D: usize    = 0x0c;    // Data
const SMI_DSR: usize  = 0x10;    // Read settings device 0
const SMI_DSW: usize  = 0x14;    // Write settings device 0
const SMI_DMC: usize  = 0x30;    // DMA control
const SMI_DCS: usize  = 0x34;    // Direct control/status
const SMI_DCA: usize  = 0x38;    // Direct address
const SMI_DCD: usize  = 0x3c;    // Direct data
const SMI_FD: usize   = 0x40;    // FIFO debug

// Clock registers on the clock manager, not the smi device
const CLK_SMI_CTL: usize = 0xb0;
const CLK_SMI_DIV: usize = 0xb4;
const CLK_PASSWD: u32    = 0x5a000000;

// Data widths
const SMI_8_BITS: usize =  0;
const SMI_16_BITS: usize = 1;
const SMI_18_BITS: usize = 2;
const SMI_9_BITS: usize =  3;

// DMA request
const DMA_SMI_DREQ: usize = 4;

use cs::CS;
use l::L;
use a::A;
use dsr::DSR;
use dsw::DSW;
use dmc::DMC;
use dcs::DCS;
use dca::DCA;
use dcd::DCD;


pub struct Smi {
    dma: Dma,
    clk_map: MmapMut,
    smi_map: Rc<RefCell<MmapMut>>,

    cs: CS,
    l: L,
    a: A,
    dmc: DMC,
    dsr: DSR,
    dsw: DSW,
    dcs: DCS,
    dca: DCA,
    dcd: DCD,
}

impl Smi {
    pub fn new(
        width_bits: usize,
        ns: usize,
        setup: usize,
        strobe: usize,
        hold: usize,
        pace: usize,
        dma_channel: u8,
    ) -> Self {
        let width = match width_bits {
            8 => SMI_8_BITS,
            16 => SMI_16_BITS,
            18 => SMI_18_BITS,
            9 => SMI_9_BITS,
            _ => panic!("Invalid SMI data width"),
        };

        let dma = Dma::new(dma_channel);

        let devmem = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")
            .expect("Failed to open /dev/mem");

        let mut clk_map = unsafe {
            MmapOptions::new()
                .offset(CLK_BASE_ADDRESS as u64)
                .len(0x1000)
                .map_mut(&devmem)
                .expect("Failed to map CLK memory")
        };

        let smi_map = unsafe {
            MmapOptions::new()
                .offset(SMI_BASE_ADDRESS as u64)
                .len(0x1000)
                .map_mut(&devmem)
                .expect("Failed to map SMI memory")
        };
        let smi_map = Rc::new(RefCell::new(smi_map));

        let cs = CS::new(smi_map.clone());
        let l = L::new(smi_map.clone());
        let a = A::new(smi_map.clone());
        let dmc = DMC::new(smi_map.clone());
        let dsr = DSR::new(smi_map.clone(), 0);
        let dsw = DSW::new(smi_map.clone(), 0);
        let dcs = DCS::new(smi_map.clone());
        let dca = DCA::new(smi_map.clone());
        let dcd = DCD::new(smi_map.clone());

        let clk_smi_ctl = unsafe {
            clk_map.as_mut_ptr().byte_add(CLK_SMI_CTL) as *mut u32
        };

        let clk_smi_div = unsafe {
            clk_map.as_mut_ptr().byte_add(CLK_SMI_DIV) as *mut u32
        };

        // set up the clocks for the smi peripheral
        let divi = (ns / 2) as u32;

        // kill the clock and wait for it to stop
        w(clk_smi_ctl, CLK_PASSWD | (1 << 5));
        while r(clk_smi_ctl) & (1 << 7) != 0 {}

        // set clock source to plld_per which should be 500MHz
        w(clk_smi_ctl, CLK_PASSWD | 6);

        // set the divisor, the smi divisor is odd in that the bottom 8 bits are
        // fixed to 0 and can't be set, so we need to shift the divisor by 8
        w(clk_smi_div, CLK_PASSWD | (divi << 8));

        // enable the clock and wait for it to be ready
        rwm(clk_smi_ctl, |reg| *reg |= CLK_PASSWD | (1 << 4));
        while r(clk_smi_ctl) & (1 << 7) == 0 {}
        
        // clear any errors on the SMI peripheral
        if cs.get_seterr() {
            cs.set_seterr(true);
        }

        dsr.set_rsetup(setup as u8);
        dsw.set_wsetup(setup as u8);
        dsr.set_rstrobe(strobe as u8);
        dsw.set_wstrobe(strobe as u8);
        dsr.set_rhold(hold as u8);
        dsw.set_whold(hold as u8);
        dsr.set_rwidth(width as u8);
        dsw.set_wwidth(width as u8);
        dsr.set_rpace(pace as u8);
        dsw.set_wpace(pace as u8);
        dmc.set_panicr(8);
        dmc.set_panicw(8);
        dmc.set_reqr(REQUEST_THRESH as u8);
        dmc.set_reqw(REQUEST_THRESH as u8);
        if width == SMI_8_BITS {
            dsw.set_wswap(true);
        }

        Smi {
            dma,
            clk_map,
            smi_map,

            cs,
            l,
            a,
            // d,
            dmc,
            dsr,
            dsw,
            dcs,
            dca,
            dcd,
        }
    }

    pub fn setup_transfer(&mut self, source: &VcMem, range: Range<usize>) {
        /*
            txdata = (TXDATA_T *)(cbs+1);
            smi_dmc->dmaen = 1;
            smi_cs->enable = 1;
            smi_cs->clear = 1;
            smi_cs->pxldat = 1;
            smi_l->len = nsamp * sizeof(TXDATA_T);
            smi_cs->write = 1;
            enable_dma(DMA_CHAN);
            cbs[0].ti = DMA_DEST_DREQ | (DMA_SMI_DREQ << 16) | DMA_CB_SRCE_INC | DMA_WAIT_RESP;
            cbs[0].tfr_len = nsamp * sizeof(TXDATA_T);
            cbs[0].srce_ad = MEM_BUS_ADDR(mp, txdata);
            cbs[0].dest_ad = REG_BUS_ADDR(smi_regs, SMI_D);
        */

        let smi_d_bus_addr = PERIPHERAL_BUS_ADDRESS + 
            ((SMI_BASE_ADDRESS + SMI_D) - PERIPHERAL_BASE_ADDRESS);
        debug!("smi_d_bus_addr: {:x}", smi_d_bus_addr);

        let len = (range.end - range.start) as u32;
        self.dmc.set_dmaen(true);
        self.cs.set_enable(true);
        self.cs.set_clear(true);
        self.cs.set_pxldat(true);
        self.a.set_addr(0);
        self.l.set_len(len);
        self.cs.set_write(true);
        let cb = self.dma.get_cb();
        cb.set_transfer_info((
            DMA_DEST_DREQ | (DMA_SMI_DREQ << 16) | DMA_CB_SRCE_INC | DMA_WAIT_RESP
        ) as u32);
        cb.set_transfer_length(len);
        cb.set_source_address((source.busaddr() + range.start) as u32);
        cb.set_destination_address(smi_d_bus_addr as u32);
        self.dma.enable();
    }

    pub fn start_transfer(&mut self) {
        self.dma.start();
        self.cs.set_start(true);
    }

    pub fn wait_transfer(&self) {
        self.dma.wait();
        debug!("post-transfer value: {:32b}", self.cs.get_value());
    }
}