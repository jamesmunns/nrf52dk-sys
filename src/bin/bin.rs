#![no_std]
#![no_main]

// TODO: remove?
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate smooth_blue;

#[no_mangle]
pub unsafe extern fn main() {
    // println!("Hello, world!");
        // Basic test that I can call the C code (and it is linked correctly)
        // NOTE: this probably still isn't a good test until I can verify linking
        let mut x = 0;
        x = smooth_blue::nrf_log_init(None);
        x = smooth_blue::app_timer_init();

        // x = bsp_init(smooth_blue::BSP_INIT_LED | smooth_blue::BSP_INIT_BUTTONS, 0);

        // x = bsp_btn_ble_init(0, 0);

        // g_erase_bonds = (startup_event == BSP_EVENT_CLEAR_BONDING_DATA);

}
