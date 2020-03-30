use super::led::Led;
use msp432_razcal::gpio::*;

pub struct RgbLed<T: GpioPinOutput> {
    red: Led<T>,
    green: Led<T>,
    blue: Led<T>,
}

pub enum RgbLedColor {
    Off,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl<T: GpioPinOutput + GpioPortSync> RgbLed<T> {
    pub fn new(red: T, green: T, blue: T) -> Self {
        let mut led = RgbLed {
            red: Led::new(red),
            green: Led::new(green),
            blue: Led::new(blue),
        };

        led.set_color(RgbLedColor::Off);
        led
    }

    pub fn set_color(&mut self, color: RgbLedColor) {
        match color {
            RgbLedColor::Off => {
                self.red.off();
                self.green.off();
                self.blue.off();
            }

            RgbLedColor::Red => {
                self.red.on();
                self.green.off();
                self.blue.off();
            }

            RgbLedColor::Green => {
                self.red.off();
                self.green.on();
                self.blue.off();
            }

            RgbLedColor::Yellow => {
                self.red.on();
                self.green.on();
                self.blue.off();
            }

            RgbLedColor::Blue => {
                self.red.off();
                self.green.off();
                self.blue.on();
            }

            RgbLedColor::Magenta => {
                self.red.on();
                self.green.off();
                self.blue.on();
            }

            RgbLedColor::Cyan => {
                self.red.off();
                self.green.on();
                self.blue.on();
            }

            RgbLedColor::White => {
                self.red.on();
                self.green.on();
                self.blue.on();
            }
        }
    }
}
