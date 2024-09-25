#![allow(dead_code)]

use std::fs::OpenOptions;
use std::ptr::{read_volatile, write_volatile};

use memmap2::{MmapMut, MmapOptions};

use crate::GPIO_BASE_ADDRESS;

const GPIO_MODE0: usize     = 0x00;
const GPIO_SET0: usize      = 0x1c;
const GPIO_CLR0: usize      = 0x28;
const GPIO_LEV0: usize      = 0x34;
const GPIO_GPPUD: usize     = 0x94;
const GPIO_GPPUDCLK0: usize = 0x98;

pub enum GpioMode {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}

pub struct Gpio {
    mapping: MmapMut,
    configured_pins: Vec<usize>,
}

impl Gpio {
    pub fn new() -> Self {
        let devmem = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")
            .expect("Failed to open /dev/mem");

        let gpio_map = unsafe {
            MmapOptions::new()
                .offset(GPIO_BASE_ADDRESS as u64)
                .len(0x1000)
                .map_mut(&devmem)
                .expect("Failed to map GPIO peripheral")
        };

        let configured_pins = Vec::new();

        Gpio {
            mapping: gpio_map,
            configured_pins,
        }
    }

    fn set_pin_mode(&mut self, pin: usize, mode: u32) {
        let pin_offset = pin / 10;
        let shift = (pin % 10) * 3;
        let mask = 0b111 << shift;

        let reg = unsafe {
            self.mapping
                .as_mut_ptr()
                .byte_add(GPIO_MODE0)
                .add(pin_offset) as *mut u32
        };

        unsafe {
            let mut pre = read_volatile(reg);
            pre = (pre & !mask) | (mode << shift);
            write_volatile(reg, pre);
        }
    }

    pub fn configure_pin(&mut self, pin: usize, mode: GpioMode) {
        self.configured_pins.push(pin);
        match mode {
            GpioMode::Input => self.set_pin_mode(pin, 0),
            GpioMode::Output => self.set_pin_mode(pin, 1),
            GpioMode::Alt0 => self.set_pin_mode(pin, 4),
            GpioMode::Alt1 => self.set_pin_mode(pin, 5),
            GpioMode::Alt2 => self.set_pin_mode(pin, 6),
            GpioMode::Alt3 => self.set_pin_mode(pin, 7),
            GpioMode::Alt4 => self.set_pin_mode(pin, 3),
            GpioMode::Alt5 => self.set_pin_mode(pin, 2),
        }
    }

    pub fn set_pin(&mut self, pin: usize, value: bool) {
        let pin_offset = pin / 32;
        let shift = pin % 32;

        let reg = unsafe {
            self.mapping
                .as_mut_ptr()
                .byte_add(if value { GPIO_SET0 } else { GPIO_CLR0 })
                .add(pin_offset) as *mut u32
        };

        unsafe {
            let mut pre = read_volatile(reg);
            pre = pre | (1 << shift);
            write_volatile(reg, pre);
        }
    }

    pub fn read_pin(&self, pin: usize) -> bool {
        let pin_offset = pin / 32;
        let shift = pin % 32;

        let reg = unsafe {
            self.mapping
                .as_ptr()
                .byte_add(GPIO_LEV0)
                .add(pin_offset) as *const u32
        };

        unsafe { read_volatile(reg) & (1 << shift) != 0 }
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        let pins = self.configured_pins.drain(..).collect::<Vec<_>>();
        for pin in pins {
            self.set_pin_mode(pin, 0);
        }
    }
}