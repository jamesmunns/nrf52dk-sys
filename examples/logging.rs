// Based on nRF5-sdk/examples/peripheral/blinky/main.c
#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate smooth_blue;
use smooth_blue as nrf;

#[no_mangle]
pub unsafe extern "C" fn main() {

    nrf::bsp_board_leds_init();
    nrf::check(nrf::nrf_log_init(None)).unwrap();
    // nrf::check(nrf::app_timer_init()).unwrap();

    loop {
        for led in 0..nrf::LEDS_NUMBER {
            nrf::bsp_board_led_invert(led);
            nrf::_nrf_delay_ms(500);
            log_str("INFO: this is a test\r\n\x00");
            process_log();
        }
    }
}

unsafe fn process_log() {
    loop {
        let x = nrf::nrf_log_frontend_dequeue();
        if !x {
            break
        }
    }
}

unsafe fn log_str(foo: &'static str) {
    nrf::nrf_log_frontend_std_0(nrf::NRF_LOG_LEVEL_INFO as u8, foo.as_ptr());
}

// NRF_LOG_PROCESS()
//     NRF_LOG_INTERNAL_PROCESS()
//         nrf_log_frontend_dequeue()

// NRF_LOG_INTERNAL_FLUSH

// NRF_LOG_INFO
//     NRF_LOG_INTERNAL_INFO( __VA_ARGS__)
//         LOG_INTERNAL(NRF_LOG_LEVEL_INFO, LOG_INFO_PREFIX, __VA_ARGS__);
//             LOG_INTERNAL_X(NUM_VA_ARGS_LESS_1( \
//                                                            __VA_ARGS__), type, prefix, __VA_ARGS__)
//                 CONCAT_2(LOG_INTERNAL_, N) (__VA_ARGS__)
//                     nrf_log_frontend_std_0(type, prefix str)