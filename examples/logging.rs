// Based on nRF5-sdk/examples/peripheral/blinky/main.c
#![no_std]
#![no_main]

extern crate panic_halt;

#[macro_use]
use nrf52dk_sys as nrf;
use cortex_m_rt::entry;

#[entry]
unsafe fn main() -> ! {
    nrf::bsp_board_leds_init();
    nrf::check(nrf::nrf_log_init(None)).unwrap();
    nrf::check(nrf::app_timer_init()).unwrap();

    loop {
        for led in 0..nrf::LEDS_NUMBER {
            nrf::bsp_board_led_invert(led);
            nrf::_nrf_delay_ms(500);
            log_str("INFO: this is a test\r\n\x00");
            nrf::sd_app_evt_wait();
            process_log();
        }
    }
}

unsafe fn process_log() {
    loop {
        nrf::sd_app_evt_wait();
        let x = nrf::nrf_log_frontend_dequeue();
        if !x {
            break;
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
//                            __VA_ARGS__), type, prefix, __VA_ARGS__)
//                 CONCAT_2(LOG_INTERNAL_, N) (__VA_ARGS__)
//                     nrf_log_frontend_std_0(type, prefix str)
