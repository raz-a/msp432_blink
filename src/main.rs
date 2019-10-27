
#![no_std]
#![no_main]
#![feature(asm)]

mod chip;
mod board;

use board::led::*;
use chip::gpio::*;
use chip::pin::Pin;
use chip::watchdog::WatchdogTimer;
use core::panic::PanicInfo;
use core::ptr;

#[inline(never)]
fn main() -> ! {
    let pin = Pin::new(RGB_BLUE_LED_PIN);
    match pin {
        Some(p) => {
            let mut led = Led::new(PushPullGpioOut::new(p));
            loop {
                delay(500000);
                led.toggle();
            }
        },

        None => {
            debug_assert!(false);
        }
    }

    loop {};
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
        },

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
    loop{};
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTION_VECTORS: [unsafe extern "C" fn() -> !; 14] = [default_handler; 14];

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static INTERRUPT_VECTORS: [unsafe extern "C" fn() -> !; 41] = [default_handler; 41];
