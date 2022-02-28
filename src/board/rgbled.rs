#![allow(dead_code)]

use super::led::Led;
use msp432_razcal::gpio::*;

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
// Contiguous Pins.
//

pub struct RgbLed<Bus: GpioBusOutput<3>> {
    pins: Bus,
}

impl<Bus: GpioBusOutput<3>> RgbLed<Bus> {
    pub fn new(pins: Bus) -> Self {
        let mut led = Self { pins: pins };
        led.set_color(RgbLedColor::Off);
        led
    }

    pub fn set_color(&mut self, color: RgbLedColor) {
        self.pins.write(color.to_bits());
    }
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}
