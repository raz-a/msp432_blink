
#[derive(Copy, Clone)]
pub enum Pin {
    P2_0 = 0x01,
    P2_1 = 0x02,
    P2_2 = 0x04,
    P2_3 = 0x08,
    P2_4 = 0x10,
    P2_5 = 0x20,
    P2_6 = 0x40,
    P2_7 = 0x80,
}

pub struct PinToken {
    pin: Pin
}

static mut PORT2_PINS_AVAILABLE: u8 = 0xFF;

impl PinToken {
    pub fn new(pin: Pin) -> Option<Self> {
        let pin_mask = pin as u8;
        unsafe {
            if PORT2_PINS_AVAILABLE & pin_mask == 0 {
                return None;
            }

            PORT2_PINS_AVAILABLE &= !pin_mask;
        }

        Some(PinToken {pin: pin})
    }

    pub fn get_pin(&self) -> Pin {
        self.pin
    }
}

impl Drop for PinToken {
    fn drop(&mut self) {
        let pin_mask = self.pin as u8;
        unsafe {
            PORT2_PINS_AVAILABLE |= pin_mask;
        }
    }
}