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