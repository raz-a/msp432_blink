

use crate::chip::pin::{Pin, PinName};
use core::mem::size_of;

const P2OUT_BITBAND: usize = 0x4209_8060;
const P2DIR_BITBAND: usize = 0x4209_80A0;

const BITBAND_RED_LED_VALUE: usize = P2OUT_BITBAND;
const BITBAND_RED_LED_DIR: usize = P2DIR_BITBAND;

const BITBAND_GREEN_LED_VALUE: usize = BITBAND_RED_LED_VALUE + size_of::<u32>();
const BITBAND_GREEN_LED_DIR: usize = BITBAND_RED_LED_DIR + size_of::<u32>();

const BITBAND_BLUE_LED_VALUE: usize = BITBAND_GREEN_LED_VALUE + size_of::<u32>();
const BITBAND_BLUE_LED_DIR: usize = BITBAND_GREEN_LED_DIR + size_of::<u32>();

pub struct Led {
    out_addr: &'static mut u8,
    dir_addr: &'static mut u8,
    _pin: Pin
}

impl Led {
    pub fn new(pin: Pin) -> Option<Self> {
        let mut led = match pin.get_pin() {
            PinName::P2_0 => unsafe {
                Some (
                    Led {
                        out_addr: &mut *(BITBAND_RED_LED_VALUE as *mut u8),
                        dir_addr: &mut *(BITBAND_RED_LED_DIR as *mut u8),
                        _pin: pin
                    }
                )
            },

            PinName::P2_1 => unsafe {
                Some (
                    Led {
                        out_addr: &mut *(BITBAND_GREEN_LED_VALUE as *mut u8),
                        dir_addr: &mut *(BITBAND_GREEN_LED_DIR as *mut u8),
                        _pin: pin
                    }
                )
            },
            PinName::P2_2 => unsafe {
                Some (
                    Led {
                        out_addr: &mut *(BITBAND_BLUE_LED_VALUE as *mut u8),
                        dir_addr: &mut *(BITBAND_BLUE_LED_DIR as *mut u8),
                        _pin: pin
                    }
                )
            },

            _ => None
        };

        match &mut led {
            Some(l) => unsafe {
                core::ptr::write_volatile(l.dir_addr, 1);
                core::ptr::write_volatile(l.out_addr, 0);
            },

            None => {}
        }

        led
    }

    pub fn on(&mut self) {
        unsafe {
            core::ptr::write_volatile(self.out_addr, 1);
        }
    }

    pub fn off(&mut self) {
        unsafe {
            core::ptr::write_volatile(self.out_addr, 0);
        }
    }

    pub fn toggle(&mut self) {
        let led_value = unsafe {
            core::ptr::read_volatile(self.out_addr)
        };

        unsafe {
            core::ptr::write_volatile(self.out_addr, led_value ^ 1);
        }
    }
}
