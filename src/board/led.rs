

use msp432_razcal::pin::PinName;
use msp432_razcal::gpio::GpioOut;

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
