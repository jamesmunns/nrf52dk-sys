#![no_std]
#![no_main]

#![feature(asm)]

// TODO: remove?
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate smooth_blue;

#[no_mangle]
pub unsafe extern "C" fn main() {
    // println!("Hello, world!");
    // Basic test that I can call the C code (and it is linked correctly)
    // NOTE: this probably still isn't a good test until I can verify linking
    let mut x = 0;
    x = smooth_blue::nrf_log_init(None);
    x = smooth_blue::app_timer_init();

    init_bt();

    // Mimic example
    loop {
        if !smooth_blue::nrf_log_frontend_dequeue() {
            smooth_blue::nrf_log_frontend_std_0(smooth_blue::NRF_LOG_LEVEL_ERROR as u8,
                                                "hello world\r\n\0".as_ptr());

            // This may not work, may need to init the softdevice first
            x = smooth_blue::sd_app_evt_wait();

            x += 1;

            x as usize;
        }
    }

    // x = bsp_init(smooth_blue::BSP_INIT_LED | smooth_blue::BSP_INIT_BUTTONS, 0);

    // x = bsp_btn_ble_init(0, 0);

    // g_erase_bonds = (startup_event == BSP_EVENT_CLEAR_BONDING_DATA);

}

pub unsafe fn init_sys() {}

pub unsafe fn init_bt() {
    // /**@brief Function for initializing the BLE stack.
    //  *
    //  * @details Initializes the SoftDevice and the BLE event interrupt.
    //  */
    // static void ble_stack_init(TSDK_bluetooth_event bt_evt, TSDK_system_event sys_evt)
    // {
    //     ret_code_t err_code;

    //     nrf_clock_lf_cfg_t clock_lf_cfg = NRF_CLOCK_LFCLKSRC;
    // Low frequency clock source to be used by the SoftDevice
    // #define NRF_CLOCK_LFCLKSRC      {.source        = NRF_CLOCK_LF_SRC_XTAL,            \
    //                                  .rc_ctiv       = 0,                                \
    //                                  .rc_temp_ctiv  = 0,                                \
    //                                  .xtal_accuracy = NRF_CLOCK_LF_XTAL_ACCURACY_20_PPM}

    let mut clk_cfg = smooth_blue::nrf_clock_lf_cfg_t {
        source: smooth_blue::NRF_CLOCK_LF_SRC_XTAL as u8,
        rc_ctiv: 0,
        rc_temp_ctiv: 0,
        xtal_accuracy: smooth_blue::NRF_CLOCK_LF_XTAL_ACCURACY_20_PPM as u8,
    };

    // //     // Initialize the SoftDevice handler module.
    // //     SOFTDEVICE_HANDLER_INIT(&clock_lf_cfg, NULL);
    //         // static uint32_t BLE_EVT_BUFFER[CEIL_DIV(BLE_STACK_EVT_MSG_BUF_SIZE, sizeof(uint32_t))];    \
    //         // uint32_t ERR_CODE;                                                                         \
    //         // ERR_CODE = softdevice_handler_init((CLOCK_SOURCE),                                         \
    //         //                                    BLE_EVT_BUFFER,                                         \
    //         //                                    sizeof(BLE_EVT_BUFFER),                                 \
    //         //                                    EVT_HANDLER);                                           \
    //         // APP_ERROR_CHECK(ERR_CODE);
    let mut x: [u8; 100] = [0; 100]; //*mut _ as *mut c_void
    let y = smooth_blue::softdevice_handler_init(&mut clk_cfg as
                                                 *mut smooth_blue::nrf_clock_lf_cfg_t,
                                                 &mut x[0] as *mut _ as
                                                 *mut smooth_blue::ctypes::c_void,
                                                 100,
                                                 None);
    assert_eq!(0, y);

    //     // Fetch the start address of the application RAM.
    //     uint32_t ram_start = 0;
    //     err_code = softdevice_app_ram_start_get(&ram_start);
    //     APP_ERROR_CHECK(err_code);

    let mut ram_start = 0u32;
    let y = smooth_blue::softdevice_app_ram_start_get(&mut ram_start);
    assert_eq!(0, y);

    //     // Overwrite some of the default configurations for the BLE stack.
    //     ble_cfg_t ble_cfg;
    //     memset(&ble_cfg, 0, sizeof(ble_cfg));
    //     ble_cfg.common_cfg.vs_uuid_cfg.vs_uuid_count = 0;

    // I'm so bad. So so bad.
    let mut ble_cfg = core::mem::transmute::<[u8; 24], smooth_blue::ble_cfg_t>([0u8; 24]);



    //     //AJM err_code = sd_ble_cfg_set(BLE_COMMON_CFG_VS_UUID, &ble_cfg, ram_start);
    //     //AJM APP_ERROR_CHECK(err_code);
    let y = smooth_blue::sd_ble_cfg_set(smooth_blue::BLE_COMMON_CFGS::BLE_COMMON_CFG_VS_UUID as
                                        u32,
                                        &mut ble_cfg,
                                        ram_start);
    assert_eq!(0, y);

    //     // Configure the maximum number of connections.
    //     memset(&ble_cfg, 0, sizeof(ble_cfg));
    //     ble_cfg.gap_cfg.role_count_cfg.periph_role_count  = BLE_GAP_ROLE_COUNT_PERIPH_DEFAULT;
    //     ble_cfg.gap_cfg.role_count_cfg.central_role_count = 0;
    //     ble_cfg.gap_cfg.role_count_cfg.central_sec_count  = 0;
    //     err_code = sd_ble_cfg_set(BLE_GAP_CFG_ROLE_COUNT, &ble_cfg, ram_start);
    //     APP_ERROR_CHECK(err_code);
    // AJM TODO: figure out how unions work
    let foo: [u8; 24] = [smooth_blue::BLE_GAP_ROLE_COUNT_PERIPH_DEFAULT as u8, 0, 0, 0,
                         0, 0, 0, 0,
                         0, 0, 0, 0,
                         0, 0, 0, 0,
                         0, 0, 0, 0,
                         0, 0, 0, 0,];
    ble_cfg = core::mem::transmute::<[u8; 24], smooth_blue::ble_cfg_t>(foo);
    let y = smooth_blue::sd_ble_cfg_set(smooth_blue::BLE_GAP_CFGS::BLE_GAP_CFG_ROLE_COUNT as u32,
                                        &mut ble_cfg,
                                        ram_start);
    assert_eq!(0, y);

    //     // Enable BLE stack.
    //     err_code = softdevice_enable(&ram_start);
    //     APP_ERROR_CHECK(err_code);
    let y = smooth_blue::softdevice_enable(&mut ram_start);
    assert_eq!(0, y);

    //     // Register with the SoftDevice handler module for BLE events.
    //     err_code = softdevice_ble_evt_handler_set(bt_evt);
    //     APP_ERROR_CHECK(err_code);
    let y = smooth_blue::softdevice_ble_evt_handler_set(Some(bt_evt));
    assert_eq!(0, y);

    //     // Register with the SoftDevice handler module for BLE events.
    //     err_code = softdevice_sys_evt_handler_set(sys_evt);
    //     APP_ERROR_CHECK(err_code);
    let y = smooth_blue::softdevice_sys_evt_handler_set(Some(sys_evt));
    assert_eq!(0, y);

    loop {
        let _ = smooth_blue::sd_app_evt_wait();
    }

    // }

}

unsafe extern "C" fn bt_evt(p_ble_evt: *mut smooth_blue::ble_evt_t) {
    // bkpt!();
}

unsafe extern "C" fn sys_evt(evt_id: u32) {
    // bkpt!();
}