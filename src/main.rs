

use std::thread;
use std::time::Duration;

use flexi_logger::{colored_with_thread, Logger, WriteMode};
use log::{error, info};

mod dma;
mod vc_mem;
mod gpio;
mod smi;

use smi::Smi;
use vc_mem::VcMem;
use gpio::{Gpio, GpioMode};

const PERIPHERAL_BUS_ADDRESS: usize = 0x7E000000;
const PERIPHERAL_BASE_ADDRESS: usize = 0x3F000000;
const DMA_BASE_ADDRESS: usize = PERIPHERAL_BASE_ADDRESS + 0x007000;
const CLK_BASE_ADDRESS: usize = PERIPHERAL_BASE_ADDRESS + 0x101000;
const GPIO_BASE_ADDRESS: usize = PERIPHERAL_BASE_ADDRESS + 0x200000;
const SMI_BASE_ADDRESS: usize = PERIPHERAL_BASE_ADDRESS + 0x600000;

const LED_D0_PIN: usize     =  8;   // GPIO pin for D0 output
const LED_NCHANS: usize     =  8;   // Number of LED channels (8 or 16)
const LED_NBITS: usize      =  24;  // Number of data bits per LED
const LED_PREBITS: usize    =  0;   // Number of zero bits before LED data
const LED_POSTBITS: usize   =  100;   // Number of zero bits after LED data
const BIT_NPULSES: usize    =  3;   // Number of O/P pulses per LED bit
const CHAN_MAXLEDS: usize   =  128; // Maximum number of LEDs per channel. NOTE: more than 450 isnt possible somehow.
const REQUEST_THRESH: usize =  2;   // DMA request threshold
const DMA_CHAN: usize       =  10;  // DMA channel to use

// Length of data for 1 row (1 LED on each channel)
const LED_DLEN: usize = LED_NBITS * BIT_NPULSES;

const fn led_tx_offset(n: usize) -> usize { LED_PREBITS + (LED_DLEN * (n)) }
const fn tx_buff_len(n: usize) -> usize { led_tx_offset(n) + LED_POSTBITS }
const fn tx_buff_size(n: usize) -> usize { 
    tx_buff_len(n) * std::mem::size_of::<u8>()
}
const VC_MEM_SIZE: usize = tx_buff_size(CHAN_MAXLEDS) + 0xFFF & !0xFFF;

// how many LEDs are actually in each channel
const CHAN_LED_COUNT: usize = 2;

// some short helpers for reading and writing volatile memory since we do it
// a lot in this code
#[inline(always)]
fn r<T>(addr: *const T) -> T {
    unsafe { addr.read_volatile() }
}
#[inline(always)]
fn w<T>(addr: *mut T, val: T) {
    unsafe { addr.write_volatile(val) }
}
#[inline(always)]
fn rwm<T>(addr: *mut T, cb: impl FnOnce(&mut T)) {
    unsafe {
        let mut val = addr.read_volatile();
        cb(&mut val);
        addr.write_volatile(val);
    }
}

fn main() {
    let _logger = Logger::try_with_str("info")
        .unwrap()
        .write_mode(WriteMode::Direct)
        .format(colored_with_thread)
        .use_utc()
        .start()
        .unwrap();

    // check if running as root and exit if not
    if !is_root() {
        error!("You need to be root to run this program.");
        std::process::exit(1);
    }


    let mut gpio = Gpio::new();
    gpio.configure_pin(LED_D0_PIN, GpioMode::Alt1);

    let mut smi = Smi::new(
        8,
        160, // ns
        1, // setup
        40, // strobe
        1, // hold,
        0, // pace
        DMA_CHAN as u8
    );

    let mut tx_buff = VcMem::new(VC_MEM_SIZE as u32, 0x1000);
    smi.setup_transfer(&tx_buff, 0..tx_buff_len(CHAN_LED_COUNT));

    // initialize the LED data buffer
    let mut offset = led_tx_offset(0);
    for _ in 0..(CHAN_LED_COUNT * LED_NBITS) {
        tx_buff[offset] = 0xFF;
        tx_buff[offset + 1] = 0x00;
        tx_buff[offset + 2] = 0x00;
        offset += BIT_NPULSES;
    }

    info!("Starting LED test...");

    loop {
        info!("BLUE/RED");
        let leds = [0x0000FF, 0xFF0000];
        write_leds(&mut tx_buff, &leds);

        smi.start_transfer();
        smi.wait_transfer();

        thread::sleep(Duration::from_secs(1));

        info!("RED/BLUE");
        let leds = [0xFF0000, 0x0000FF];
        write_leds(&mut tx_buff, &leds);

        smi.start_transfer();
        smi.wait_transfer();
        thread::sleep(Duration::from_secs(1));
    }
}

fn write_leds(buf: &mut VcMem, leds: &[u32]) {
    // for now we are only writing to channel 0, expand this eventually
    const CHANNEL_MASK: u8 = 1 << 0;
    assert!(leds.len() <= CHAN_LED_COUNT);
    for (i, color) in leds.iter().enumerate() {
        let mut off = led_tx_offset(i);
        let mut rgb_mask = 1 << 23;
        for _ in 0..LED_NBITS {
            if (color & rgb_mask) != 0 {
                buf[off + 1] |= CHANNEL_MASK;
            } else {
                buf[off + 1] &= !CHANNEL_MASK;
            }
            rgb_mask >>= 1;
            off += BIT_NPULSES;
        }
    }
}

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}
