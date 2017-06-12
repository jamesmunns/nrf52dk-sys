// Based on nRF5-sdk/examples/peripheral/blinky/main.c
#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nrf52dk_sys;
use nrf52dk_sys as nrf;
use nrf::check;

#[no_mangle]
pub unsafe extern "C" fn main() {

    nrf::bsp_board_leds_init();

    loop {
        for led in 0..nrf::LEDS_NUMBER {
            nrf::bsp_board_led_invert(led);
            nrf::_nrf_delay_ms(500);
        }
    }
}
