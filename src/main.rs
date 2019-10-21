
#![no_std]
#![no_main]
#![feature(asm)]

mod led;
mod pin;
mod watchdog;

use core::panic::PanicInfo;
use core::ptr;
use led::Led;
use pin::{Pin, PinName};
use watchdog::WatchdogTimer;

#[inline(never)]
fn main() -> ! {
    let pin = Pin::new(PinName::P2_2);
    match pin {
        Some(p) => {
            let led = Led::new(p);
            match led {
                Some(mut l) => {
                    loop {
                        delay(500000);
                        l.toggle();
                    }
                },

                None => {}
            }
        },

        None => {}
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

    extern "C" {
        static mut __bss_start: u8;
        static mut __bss_end: u8;

        static mut __data_start: u8;
        static mut __data_end: u8;

        static mut __data_load_start: u8;
    }

    let mut start_bss = &__bss_start as *const u8 as usize;
    let end_bss = &__bss_end as *const u8 as usize;
    let bss_size = end_bss - start_bss;

    //
    // Explicilty avoid using ptr::write_bytes to avoid using large memclr
    //

    for _ in 0..bss_size {
        ptr::write(start_bss as *mut u8, 0);
        start_bss += 1;
    }

    let mut load_data = &__data_load_start as *const u8 as usize;
    let mut start_data = &__data_start as *const u8 as usize;
    let end_data = &__data_end as *const u8 as usize;
    let data_size = end_data - start_data;

    //
    // Explicilty avoid using ptr::copy_nonoverlapping to avoid using large memcpy
    //

    for _ in 0..data_size {
        let data = ptr::read(load_data as *const u8);
        ptr::write(start_data as *mut u8, data);
        load_data += 1;
        start_data += 1;
    }

    //
    // Disable Watchdog
    //

    let wdt = WatchdogTimer::acquire();
    match wdt {
        Some(mut timer) => {
            timer.disable();
        },

        None => {
            debug_assert!(false);
        }
    }

    main();
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