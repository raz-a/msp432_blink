#![no_std]
#![no_main]
#![feature(asm)]

extern crate msp432_razcal;
mod board;

use board::led::*;
use board::rgbled::*;
use board::*;
use core::panic::PanicInfo;
use core::ptr;
use msp432_razcal::gpio::*;
use msp432_razcal::pin::Pin;
use msp432_razcal::watchdog::WatchdogTimer;

#[inline(never)]
fn main() -> ! {
    //let red = Pin::new(RGB_RED_LED_PIN);
    //let green = Pin::new(RGB_GREEN_LED_PIN);
    let blue = Pin::new(RGB_BLUE_LED_PIN);

    match blue {
        Some(b) => {
            let mut led = Led::new(gpio_pin_new(b).to_output_pushpull());
            loop {
                led.toggle();
                delay(1000000);
            }
        }

        None => {
            debug_assert!(false);
        }
    }

    // if let (Some(red), Some(green), Some(blue)) = (red, green, blue) {
    //     let mut rgbled = RgbLed::new(
    //         gpio_pin_new(red).to_output_pushpull(),
    //         gpio_pin_new(green).to_output_pushpull(),
    //         gpio_pin_new(blue).to_output_pushpull(),
    //     );

    //     loop {
    //         rgbled.set_color(RgbLedColor::Red);
    //         delay(1000000);

    //         rgbled.set_color(RgbLedColor::Green);
    //         delay(1000000);

    //         rgbled.set_color(RgbLedColor::Yellow);
    //         delay(1000000);

    //         rgbled.set_color(RgbLedColor::Blue);
    //         delay(1000000);

    //         rgbled.set_color(RgbLedColor::Magenta);
    //         delay(1000000);

    //         rgbled.set_color(RgbLedColor::Cyan);
    //         delay(1000000);

    //         rgbled.set_color(RgbLedColor::White);
    //         delay(1000000);
    //     }
    // } else {
    //     debug_assert!(false);
    // }

    loop {}
}

fn delay(count: i32) {
    for _ in (0..count).rev() {
        unsafe {
            asm!("");
        }
    }
}

#[link_section = ".vector_table.reset"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    init_memory_sections();
    disable_watchdog();
    main();
}

unsafe fn init_memory_sections() {
    extern "C" {
        static mut __bss_start: u8;
        static mut __bss_end: u8;

        static mut __data_start: u8;
        static mut __data_end: u8;

        static mut __data_load_start: u8;
    }

    let start_bss = &__bss_start as *const u8 as usize;
    let end_bss = &__bss_end as *const u8 as usize;
    let bss_size = end_bss - start_bss;

    ptr::write_bytes(start_bss as *mut u8, 0, bss_size);

    let load_data = &__data_load_start as *const u8 as usize;
    let start_data = &__data_start as *const u8 as usize;
    let end_data = &__data_end as *const u8 as usize;
    let data_size = end_data - start_data;

    ptr::copy_nonoverlapping(load_data as *mut u8, start_data as *mut u8, data_size);
}

fn disable_watchdog() {
    let wdt = WatchdogTimer::acquire();
    match wdt {
        Some(mut timer) => {
            timer.disable();
        }

        None => {
            debug_assert!(false);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        unsafe {
            asm!("nop");
        }
    }
}

pub unsafe extern "C" fn default_handler() -> ! {
    loop {}
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTION_VECTORS: [unsafe extern "C" fn() -> !; 14] = [default_handler; 14];

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static INTERRUPT_VECTORS: [unsafe extern "C" fn() -> !; 41] = [default_handler; 41];
