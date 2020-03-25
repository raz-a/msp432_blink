use msp432_razcal::gpio::*;

pub struct Led<T: GpioPinOutput> {
    gpio_out: T,
}

impl<T: GpioPinOutput> Led<T> {
    pub fn new(gpio_out: T) -> Self {
        let mut led = Led { gpio_out: gpio_out };
        led.off();
        led
    }

    pub fn on(&mut self) {
        self.gpio_out.set();
    }

    pub fn off(&mut self) {
        self.gpio_out.clear();
    }

    pub fn toggle(&mut self) {
        self.gpio_out.toggle();
    }
}
