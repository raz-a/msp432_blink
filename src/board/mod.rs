use msp432_razcal::pin::PinName;

pub const RGB_RED_LED_PIN: PinName = PinName::P2_0;
pub const RGB_GREEN_LED_PIN: PinName = PinName::P2_1;
pub const RGB_BLUE_LED_PIN: PinName = PinName::P2_2;

pub mod led;
pub mod rgbled;
