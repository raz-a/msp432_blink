use msp432_razcal::gpio::*;

pub struct Led<Pin: GpioPinOutput> {
    gpio_pin: Pin,
}

impl<Pin: GpioPinOutput> Led<Pin> {
    pub fn new(gpio_pin: Pin) -> Self {
        let mut led = Led { gpio_pin: gpio_pin };
        led.off();
        led
    }

    pub fn on(&mut self) {
        self.gpio_pin.set();
    }

    pub fn off(&mut self) {
        self.gpio_pin.clear();
    }

    pub fn toggle(&mut self) {
        self.gpio_pin.toggle();
    }
}
