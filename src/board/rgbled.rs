#![allow(dead_code)]

use super::led::Led;
use msp432_razcal::gpio::*;

pub trait RgbLed {
    fn set_color(&mut self, color: RgbLedColor);
}

#[derive(Clone, Copy)]
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

impl RgbLedColor {
    pub fn cycle(self) -> Self {
        match self {
            RgbLedColor::Off => RgbLedColor::White,
            RgbLedColor::Red => RgbLedColor::Yellow,
            RgbLedColor::Green => RgbLedColor::Cyan,
            RgbLedColor::Yellow => RgbLedColor::Green,
            RgbLedColor::Blue => RgbLedColor::Magenta,
            RgbLedColor::Magenta => RgbLedColor::Red,
            RgbLedColor::Cyan => RgbLedColor::Blue,
            RgbLedColor::White => RgbLedColor::Off,
        }
    }

    pub fn to_bits(&self) -> usize {
        match self {
            RgbLedColor::Off => 0x0,
            RgbLedColor::Red => 0x1,
            RgbLedColor::Green => 0x2,
            RgbLedColor::Yellow => 0x3,
            RgbLedColor::Blue => 0x4,
            RgbLedColor::Magenta => 0x5,
            RgbLedColor::Cyan => 0x6,
            RgbLedColor::White => 0x7,
        }
    }
}

//
// Sparse pins.
//

pub struct RgbLedSparse<RedPin: GpioPinOutput, GreenPin: GpioPinOutput, BluePin: GpioPinOutput> {
    red: Led<RedPin>,
    green: Led<GreenPin>,
    blue: Led<BluePin>,
}

impl<RedPin: GpioPinOutput, GreenPin: GpioPinOutput, BluePin: GpioPinOutput>
    RgbLedSparse<RedPin, GreenPin, BluePin>
{
    pub fn new(red: RedPin, green: GreenPin, blue: BluePin) -> Self {
        let mut led = Self {
            red: Led::new(red),
            green: Led::new(green),
            blue: Led::new(blue),
        };

        led.set_color(RgbLedColor::Off);
        led
    }
}

impl<RedPin: GpioPinOutput, GreenPin: GpioPinOutput, BluePin: GpioPinOutput> RgbLed
    for RgbLedSparse<RedPin, GreenPin, BluePin>
{
    fn set_color(&mut self, color: RgbLedColor) {
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

//
// Contiguous Pins.
//

pub struct RgbLedContiguous<Bus: GpioBusOutput<3>> {
    pins: Bus,
}

impl<Bus: GpioBusOutput<3>> RgbLedContiguous<Bus> {
    pub fn new(pins: Bus) -> Self {
        let mut led = Self { pins: pins };
        led.set_color(RgbLedColor::Off);
        led
    }
}

impl<Bus: GpioBusOutput<3>> RgbLed for RgbLedContiguous<Bus> {
    fn set_color(&mut self, color: RgbLedColor) {
        self.pins.write(color.to_bits());
    }
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}
