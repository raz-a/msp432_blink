

use crate::chip::pin::PinName;
use crate::chip::gpio::GpioOut;

pub const RGB_RED_LED_PIN: PinName   = PinName::P2_0;
pub const RGB_GREEN_LED_PIN: PinName = PinName::P2_1;
pub const RGB_BLUE_LED_PIN: PinName  = PinName::P2_2;

pub struct Led<T: GpioOut> {
    gpio_out: T
}

impl<T: GpioOut> Led<T> {
    pub fn new(gpio_out: T) -> Self {
        let mut led = Led {gpio_out: gpio_out};
        led.off();
        led
    }

    pub fn on(&mut self) {
        self.gpio_out.set(true);
    }

    pub fn off(&mut self) {
        self.gpio_out.set(false);
    }

    pub fn toggle(&mut self) {
        self.gpio_out.toggle();
    }
}
