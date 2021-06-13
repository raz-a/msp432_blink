#![no_std]
#![no_main]
#![feature(asm)]

use core::{panic::PanicInfo, ptr};

use board::led::Led;
use msp432_razcal::{
    gpio::{GpioPin, GpioSectionBus},
    pin::{McuPinSet, PortSection3},
    watchdog::WatchdogTimer,
};

use crate::board::rgbled::{RgbLed, RgbLedColor, RgbLedContiguous};

extern crate msp432_razcal;
mod board;

#[inline(never)]
fn main() -> ! {
    let mut color = RgbLedColor::Red;
    let mcu_pins = McuPinSet::get_mcu_pins();
    if let Some(pins) = mcu_pins {
        let mut led = Led::new(GpioPin::new(pins.pa0).to_output_pushpull());
        let mut rgbled = RgbLedContiguous::new(
            GpioSectionBus::new(PortSection3::<'A', 8>::new(pins.pa8, pins.pa9, pins.pa10))
                .to_output_pushpull(),
        );

        loop {
            led.toggle();
            rgbled.set_color(color);
            color = color.cycle();
            delay(1000000);
        }
    }

    debug_assert!(false);
    loop {}
}

fn delay(count: i32) {
    for _ in 0..count {
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
