
use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Copy, Clone)]
pub enum PinName {
    P2_0 = 0x01,
    P2_1 = 0x02,
    P2_2 = 0x04,
    P2_3 = 0x08,
    P2_4 = 0x10,
    P2_5 = 0x20,
    P2_6 = 0x40,
    P2_7 = 0x80,
}

pub struct Pin {
    pin: PinName
}

static mut PORT2_PINS_AVAILABLE: AtomicU8 = AtomicU8::new(0xFF);

impl Pin {
    pub fn new(pin: PinName) -> Option<Self> {
        let pin_mask = pin as u8;
        let value = unsafe {
            PORT2_PINS_AVAILABLE.fetch_nand(pin_mask, Ordering::Relaxed)
        };

        if value & pin_mask == 0 {
            return None;
        }

        Some(Pin {pin: pin})
    }

    pub fn get_pin(&self) -> PinName {
        self.pin
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        let pin_mask = self.pin as u8;
        unsafe {
            PORT2_PINS_AVAILABLE.fetch_or(pin_mask, Ordering::Relaxed);
        }
    }
}