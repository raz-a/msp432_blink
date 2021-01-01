use msp432_razcal::pin::PinName;

#[allow(dead_code)]
pub const RGB_RED_LED_PIN: PinName = PinName::P2_0;

#[allow(dead_code)]
pub const RGB_GREEN_LED_PIN: PinName = PinName::P2_1;

#[allow(dead_code)]
pub const RGB_BLUE_LED_PIN: PinName = PinName::P2_2;

pub mod led;
pub mod rgbled;
