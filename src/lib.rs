//! Based on `cortex-m-template`
//!
//! https://github.com/japaric/cortex-m-template

// #![deny(missing_docs)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
// #![deny(warnings)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]
#![no_std]

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
    #[repr(u8)]
    pub enum c_void {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    // Non-Windows
    pub type c_long = i32;
    pub type c_ulong = u32;

    // Unsure about this one… In doubt, align cfg(all(target_os = "linux", target_arch = "arm"))
    pub type c_char = u8;
}

pub fn check(ec: u32) -> Result<(), ()> {
    match ec {
        0 => Ok(()),
        _ => Err(()),
    }
}

// We cannot use code gen here because bindgen created __va_list twice. Could
// we be double-defining things in C?
#[repr(C)]
#[derive(Debug)]
pub struct __va_list {
    pub __ap: *mut ctypes::c_void,
}

// Our own type alias to i8. We cannot use the generated code here, because
// bindgen yields a u8, and we have require constants.
pub type IRQn_Type = i8;

pub const IRQn_Type_Reset_IRQn: IRQn_Type = -15;
pub const IRQn_Type_NonMaskableInt_IRQn: IRQn_Type = -14;
pub const IRQn_Type_HardFault_IRQn: IRQn_Type = -13;
pub const IRQn_Type_MemoryManagement_IRQn: IRQn_Type = -12;
pub const IRQn_Type_BusFault_IRQn: IRQn_Type = -11;
pub const IRQn_Type_UsageFault_IRQn: IRQn_Type = -10;
pub const IRQn_Type_SVCall_IRQn: IRQn_Type = -5;
pub const IRQn_Type_DebugMonitor_IRQn: IRQn_Type = -4;
pub const IRQn_Type_PendSV_IRQn: IRQn_Type = -2;
pub const IRQn_Type_SysTick_IRQn: IRQn_Type = -1;
pub const IRQn_Type_POWER_CLOCK_IRQn: IRQn_Type = 0;
pub const IRQn_Type_RADIO_IRQn: IRQn_Type = 1;
pub const IRQn_Type_UARTE0_UART0_IRQn: IRQn_Type = 2;
pub const IRQn_Type_SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0_IRQn: IRQn_Type = 3;
pub const IRQn_Type_SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1_IRQn: IRQn_Type = 4;
pub const IRQn_Type_NFCT_IRQn: IRQn_Type = 5;
pub const IRQn_Type_GPIOTE_IRQn: IRQn_Type = 6;
pub const IRQn_Type_SAADC_IRQn: IRQn_Type = 7;
pub const IRQn_Type_TIMER0_IRQn: IRQn_Type = 8;
pub const IRQn_Type_TIMER1_IRQn: IRQn_Type = 9;
pub const IRQn_Type_TIMER2_IRQn: IRQn_Type = 10;
pub const IRQn_Type_RTC0_IRQn: IRQn_Type = 11;
pub const IRQn_Type_TEMP_IRQn: IRQn_Type = 12;
pub const IRQn_Type_RNG_IRQn: IRQn_Type = 13;
pub const IRQn_Type_ECB_IRQn: IRQn_Type = 14;
pub const IRQn_Type_CCM_AAR_IRQn: IRQn_Type = 15;
pub const IRQn_Type_WDT_IRQn: IRQn_Type = 16;
pub const IRQn_Type_RTC1_IRQn: IRQn_Type = 17;
pub const IRQn_Type_QDEC_IRQn: IRQn_Type = 18;
pub const IRQn_Type_COMP_LPCOMP_IRQn: IRQn_Type = 19;
pub const IRQn_Type_SWI0_EGU0_IRQn: IRQn_Type = 20;
pub const IRQn_Type_SWI1_EGU1_IRQn: IRQn_Type = 21;
pub const IRQn_Type_SWI2_EGU2_IRQn: IRQn_Type = 22;
pub const IRQn_Type_SWI3_EGU3_IRQn: IRQn_Type = 23;
pub const IRQn_Type_SWI4_EGU4_IRQn: IRQn_Type = 24;
pub const IRQn_Type_SWI5_EGU5_IRQn: IRQn_Type = 25;
pub const IRQn_Type_TIMER3_IRQn: IRQn_Type = 26;
pub const IRQn_Type_TIMER4_IRQn: IRQn_Type = 27;
pub const IRQn_Type_PWM0_IRQn: IRQn_Type = 28;
pub const IRQn_Type_PDM_IRQn: IRQn_Type = 29;
pub const IRQn_Type_MWU_IRQn: IRQn_Type = 32;
pub const IRQn_Type_PWM1_IRQn: IRQn_Type = 33;
pub const IRQn_Type_PWM2_IRQn: IRQn_Type = 34;
pub const IRQn_Type_SPIM2_SPIS2_SPI2_IRQn: IRQn_Type = 35;
pub const IRQn_Type_RTC2_IRQn: IRQn_Type = 36;
pub const IRQn_Type_I2S_IRQn: IRQn_Type = 37;
pub const IRQn_Type_FPU_IRQn: IRQn_Type = 38;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
