#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;
use rt::ExceptionFrame;

extern crate nrf52dk_sys;
use nrf52dk_sys as nrf;

entry!(main);

fn main() -> ! {
    unsafe {
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
}

exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
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
