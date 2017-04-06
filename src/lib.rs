//! Based on `cortex-m-template`
//!
//! https://github.com/japaric/cortex-m-template

// #![deny(missing_docs)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![deny(warnings)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

#[cfg(feature = "semihosting")]
#[macro_reexport(hprint, hprintln)]
#[macro_use]
extern crate cortex_m_semihosting;
#[macro_reexport(bkpt)]
#[macro_use]
extern crate cortex_m;
extern crate r0;

#[macro_use]
mod macros;

mod lang_items;

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

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));