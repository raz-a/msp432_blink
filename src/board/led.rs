use msp432_razcal::gpio::*;

pub struct Led<T: GpioPinOutput> {
    gpio_out: T,
}

impl<T: GpioPinOutput + GpioPortSync> Led<T> {
    pub fn new(gpio_out: T) -> Self {
        let mut led = Led { gpio_out: gpio_out };
        led.off();
        led
    }

    pub fn on(&mut self) {
        unsafe { self.gpio_out.set_no_sync() };
    }

    pub fn off(&mut self) {
        unsafe { self.gpio_out.clear_no_sync() };
    }

    pub fn toggle(&mut self) {
        unsafe { self.gpio_out.toggle_no_sync() };
    }
}
