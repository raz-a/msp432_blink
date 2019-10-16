
#![no_std]
#![no_main]
#![feature(asm)]

mod led;
mod pin;

use core::panic::PanicInfo;
use core::ptr;
use led::Led;
use pin::{Pin, PinToken};

#[inline(never)]
fn main() -> ! {
    let pin = PinToken::new(Pin::P2_2);
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

    let start_bss = &__bss_start as *const u8 as usize;
    let end_bss = &__bss_end as *const u8 as usize;
    let bss_size = end_bss - start_bss;

    ptr::write_bytes(start_bss as *mut u8, 0, bss_size);

    let load_data = &__data_load_start as *const u8 as usize;
    let start_data = &__data_start as *const u8 as usize;
    let end_data = &__data_end as *const u8 as usize;
    let data_size = end_data - start_data;

    ptr::copy_nonoverlapping(load_data as *const u8, start_data as *mut u8, data_size);

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
    let pin = PinToken::new(Pin::P2_0);
    match pin {
        Some(p) => {
            let led = Led::new(p);
            match led {
                Some(mut l) => {
                    l.on()
                },
                None => {}
            }
        },

        None => {}
    }
    loop{};
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTION_VECTORS: [unsafe extern "C" fn() -> !; 14] = [default_handler; 14];

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static INTERRUPT_VECTORS: [unsafe extern "C" fn() -> !; 240] = [default_handler; 240];
