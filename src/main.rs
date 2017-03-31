#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(linkage)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Default panic handler
#[linkage = "weak"]
#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(_args: ::core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32)
                               -> ! {
    // hprint!("panicked at '");
    // match () {
    //     #[cfg(feature = "semihosting")]
    //     () => {
    //         ::cortex_m_semihosting::io::write_fmt(_args);
    //     }
    //     #[cfg(not(feature = "semihosting"))]
    //     () => {}
    // }
    // hprintln!("', {}:{}", _file, _line);

    // bkpt!();

    loop {}
}

/// Lang item required to make the normal `main` work in applications
// This is how the `start` lang item works:
// When `rustc` compiles a binary crate, it creates a `main` function that looks
// like this:
//
// ```
// #[export_name = "main"]
// pub extern "C" fn rustc_main(argc: isize, argv: *const *const u8) -> isize {
//     start(main)
// }
// ```
//
// Where `start` is this function and `main` is the binary crate's `main`
// function.
//
// The final piece is that the entry point of our program, the reset handler,
// has to call `rustc_main`. That's covered by the `reset_handler` function in
// `src/exceptions.rs`
#[lang = "start"]
extern "C" fn start(main: fn(),
                    _argc: isize,
                    _argv: *const *const u8)
                    -> isize {
    main();

    0
}

/// Copied from https://github.com/rust-lang/rust/blob/master/src/libstd/os/raw.rs
pub mod ctypes {
    // Unconditional
    pub type c_schar = u8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    #[repr(u8)] pub enum c_void {
        #[doc(hidden)] __variant1,
        #[doc(hidden)] __variant2,
    }

    // Non-Windows
    pub type c_long = i32;
    pub type c_ulong = u32;

    // Unsure about this one… In doubt, align cfg(all(target_os = "linux", target_arch = "arm"))
    pub type c_char = u8;
}

include!(concat!(env!("OUT_DIR"), "/ble.rs"));

fn main() {
    // println!("Hello, world!");
}
