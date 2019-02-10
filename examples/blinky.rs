// Based on nRF5-sdk/examples/peripheral/blinky/main.c
#![no_std]
#![no_main]

extern crate panic_halt;
use cortex_m_rt::entry;

extern crate nrf52dk_sys;
use nrf52dk_sys as nrf;

#[entry]
unsafe fn main() -> ! {
    nrf::bsp_board_leds_init();

    loop {
        for led in 0..nrf::LEDS_NUMBER {
            nrf::bsp_board_led_invert(led);
            nrf::_nrf_delay_ms(500);
        }
    }
}
