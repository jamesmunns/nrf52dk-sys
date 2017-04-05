#![no_std]
#![no_main]

// TODO: remove?
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate smooth_blue;

use smooth_blue::exceptions::{self, Exceptions};
use smooth_blue::interrupts::{self, Interrupts};

fn main() {
    // println!("Hello, world!");
    unsafe {
        // Basic test that I can call the C code (and it is linked correctly)
        // NOTE: this probably still isn't a good test until I can verify linking
        let _ = smooth_blue::app_timer_init();
        smooth_blue::SystemInit();
        smooth_blue::SystemCoreClockUpdate();
    }
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };