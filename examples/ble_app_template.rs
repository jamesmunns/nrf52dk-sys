#![no_std]
#![no_main]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;
use rt::ExceptionFrame;

extern crate nrf52dk_sys;
use nrf::check;
use nrf52dk_sys as nrf;

static NAME: &str = "RUST-BLE";

static mut EVENT_BUFFER: [u8; 88] = [0; 88]; // 64 + 23 = 87, rounded up to next word
static mut M_CONN_HANDLE: u16 = nrf::BLE_CONN_HANDLE_INVALID as u16;
const APP_FEATURE_NOT_SUPPORTED: u16 = nrf::BLE_GATT_STATUS_ATTERR_APP_BEGIN as u16 + 2;

// static mut M_GATT : nrf_ble_gatt_t = unsafe {core::mem::zeroed::<nrf_ble_gatt_t>()};
static mut M_GATT: nrf::nrf_ble_gatt_t = nrf::nrf_ble_gatt_t {
    att_mtu_desired_periph: 0,
    att_mtu_desired_central: 0,
    data_length: 0,
    links: [
        nrf::nrf_ble_gatt_link_t {
            att_mtu_desired: 0,
            att_mtu_effective: 0,
            att_mtu_exchange_pending: false,
            att_mtu_exchange_requested: false,
            data_length_desired: 0,
            data_length_effective: 0,
        },
        nrf::nrf_ble_gatt_link_t {
            att_mtu_desired: 0,
            att_mtu_effective: 0,
            att_mtu_exchange_pending: false,
            att_mtu_exchange_requested: false,
            data_length_desired: 0,
            data_length_effective: 0,
        },
    ],
    evt_handler: None,
};

static mut M_ADV_UUIDS: [nrf::ble_uuid_t; 1] = [nrf::ble_uuid_t {
    uuid: nrf::BLE_UUID_DEVICE_INFORMATION_SERVICE as u16,
    type_: nrf::BLE_UUID_TYPE_BLE as u8,
}];

unsafe fn nrf_log_info(output: &'static str) {
    nrf::nrf_log_frontend_std_0(nrf::NRF_LOG_LEVEL_INFO as u8, output.as_ptr());
}

entry!(main);

fn main() -> ! {
    unsafe {
        let mut erase_bonds = false;

        // BSP Init
        log_init();
        nrf_log_info("Hello, nRF52!\r\n\0");
        timers_init();
        buttons_leds_init(&mut erase_bonds);

        // BLE Init
        ble_stack_init();
        gap_params_init();
        gatt_init();
        advertising_init();
        services_init();
        conn_params_init();
        peer_manager_init();

        application_timers_start();

        advertising_start(erase_bonds);

        // Mimic example
        loop {
            if !nrf::nrf_log_frontend_dequeue() {
                nrf::sd_app_evt_wait();
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

/// Function for initializing the nrf log module.
unsafe fn log_init() {
    check(nrf::nrf_log_init(None)).unwrap();
}

/// Function for the Timer initialization.
///
/// Initializes the timer module. This creates and starts application timers.
///
unsafe fn timers_init() {
    check(nrf::app_timer_init()).unwrap();
}

/// Function for initializing buttons and leds.
unsafe fn buttons_leds_init(erase_bonds: &mut bool) {
    let mut startup_event: nrf::bsp_event_t = nrf::bsp_event_t_BSP_EVENT_NOTHING;

    check(nrf::bsp_init(
        nrf::BSP_INIT_LED | nrf::BSP_INIT_BUTTONS,
        Some(bsp_event_handler),
    )).unwrap();

    check(nrf::bsp_btn_ble_init(None, &mut startup_event)).unwrap();

    *erase_bonds = startup_event == nrf::bsp_event_t_BSP_EVENT_CLEAR_BONDING_DATA;
}

/// Function for initializing the BLE stack.
///
/// Initializes the SoftDevice and the BLE event interrupt.
///
unsafe fn ble_stack_init() {
    // Initialize the SoftDevice handler module.
    let mut clk_cfg = nrf::_NRF_CLOCK_LFCLKSRC();

    check(nrf::softdevice_handler_init(
        &mut clk_cfg,
        EVENT_BUFFER.as_mut_ptr() as *mut nrf::ctypes::c_void,
        EVENT_BUFFER.len() as u16,
        None,
    )).unwrap();

    // Fetch the start address of the application RAM.
    let mut ram_start = 0u32;
    check(nrf::softdevice_app_ram_start_get(&mut ram_start)).unwrap();

    // Overwrite some of the default configurations for the BLE stack.
    let mut ble_cfg = nrf::ble_cfg_t::default();
    check(nrf::sd_ble_cfg_set(
        nrf::BLE_COMMON_CFGS_BLE_COMMON_CFG_VS_UUID as u32,
        &mut ble_cfg,
        ram_start,
    )).unwrap();

    // Configure the maximum number of connections.
    let mut ble_cfg = nrf::ble_cfg_t::default();
    ble_cfg.gap_cfg.role_count_cfg = nrf::ble_gap_cfg_role_count_t {
        periph_role_count: nrf::BLE_GAP_ROLE_COUNT_PERIPH_DEFAULT as u8,
        central_role_count: 0,
        central_sec_count: 0,
    };

    check(nrf::sd_ble_cfg_set(
        nrf::BLE_GAP_CFGS_BLE_GAP_CFG_ROLE_COUNT as u32,
        &mut ble_cfg,
        ram_start,
    )).unwrap();

    // Enable BLE stack.
    check(nrf::softdevice_enable(&mut ram_start)).unwrap();

    // Register with the SoftDevice handler module for BLE events.
    check(nrf::softdevice_ble_evt_handler_set(Some(ble_evt_dispatch))).unwrap();

    // Register with the SoftDevice handler module for SYS events.
    check(nrf::softdevice_sys_evt_handler_set(Some(sys_evt_dispatch))).unwrap();
}

/// Function for the GAP initialization.
///
/// This function sets up all the necessary GAP (Generic Access Profile) parameters of the
/// device including the device name, appearance, and the preferred connection parameters.
///
unsafe fn gap_params_init() {
    // NOTE: Same as BLE_GAP_CONN_SEC_MODE_SET_OPEN macro
    let mut sec_mode = nrf::ble_gap_conn_sec_mode_t::default();
    sec_mode.set_sm(1);
    sec_mode.set_lv(1);

    check(nrf::sd_ble_gap_device_name_set(
        &mut sec_mode,
        NAME.as_ptr(),
        NAME.len() as u16,
    )).unwrap();

    //  YOUR_JOB: Use an appearance value matching the application's use case.
    //  err_code = sd_ble_gap_appearance_set(BLE_APPEARANCE_);
    //  APP_ERROR_CHECK(err_code);

    // AJM - todo, better constants
    let mut gap_conn_params = nrf::ble_gap_conn_params_t {
        min_conn_interval: 80,
        max_conn_interval: 160,
        slave_latency: 0,
        conn_sup_timeout: 400,
    };

    check(nrf::sd_ble_gap_ppcp_set(&mut gap_conn_params)).unwrap();
}

/// Function for initializing the GATT module.
unsafe fn gatt_init() {
    check(nrf::nrf_ble_gatt_init(&mut M_GATT, None)).unwrap();
}

/// Function for initializing the Advertising functionality.
unsafe fn advertising_init() {
    let mut advdata = nrf::ble_advdata_t::default();

    advdata.name_type = nrf::ble_advdata_name_type_t_BLE_ADVDATA_FULL_NAME;
    advdata.include_appearance = true;
    advdata.flags = nrf::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8;
    advdata.uuids_complete.uuid_cnt = M_ADV_UUIDS.len() as u16;
    advdata.uuids_complete.p_uuids = M_ADV_UUIDS.as_mut_ptr();

    let mut options = nrf::ble_adv_modes_config_t::default();
    options.ble_adv_fast_enabled = true;
    options.ble_adv_fast_interval = 64;
    options.ble_adv_fast_timeout = 180;

    check(nrf::ble_advertising_init(
        &mut advdata,
        core::ptr::null(),
        &mut options,
        Some(on_adv_evt),
        None,
    )).unwrap();
}

/// Function for initializing services that will be used by the application.
unsafe fn services_init() {
    // YOUR_JOB: Add code to initialize the services used by the application.
}

/// Function for initializing the Connection Parameters module.
unsafe fn conn_params_init() {
    let mut cp_init = nrf::ble_conn_params_init_t {
        p_conn_params: core::ptr::null_mut(),
        first_conn_params_update_delay: 5 * 32768,
        next_conn_params_update_delay: 30 * 32768,
        max_conn_params_update_count: 3,
        start_on_notify_cccd_handle: nrf::BLE_GATT_HANDLE_INVALID as u16,
        disconnect_on_fail: false,
        evt_handler: Some(on_conn_params_evt),
        error_handler: Some(conn_params_error_handler),
    };

    check(nrf::ble_conn_params_init(&mut cp_init)).unwrap();
}

/// Function for the Peer Manager initialization
unsafe fn peer_manager_init() {
    check(nrf::pm_init()).unwrap();

    let mut kdist_own = nrf::ble_gap_sec_kdist_t::default();
    kdist_own.set_enc(1);
    kdist_own.set_id(1);

    let mut kdist_peer = nrf::ble_gap_sec_kdist_t::default();
    kdist_peer.set_enc(1);
    kdist_peer.set_id(1);

    // Security parameters to be used for all security procedures.
    let mut sec_param = nrf::ble_gap_sec_params_t {
        _bitfield_1: nrf::__BindgenBitfieldUnit::<[u8; 1], u8>::new([0]),
        min_key_size: 7,
        max_key_size: 16,
        kdist_own: kdist_own,
        kdist_peer: kdist_peer,
    };

    sec_param.set_bond(1);
    sec_param.set_io_caps(nrf::BLE_GAP_IO_CAPS_NONE as u8);

    check(nrf::pm_sec_params_set(&mut sec_param)).unwrap();

    check(nrf::pm_register(Some(pm_evt_handler))).unwrap();
}

/// Function for starting timers.
unsafe fn application_timers_start() {
    // YOUR_JOB: Start your timers.
}

// static void bsp_event_handler(bsp_event_t event)
unsafe extern "C" fn bsp_event_handler(event: nrf::bsp_event_t) {
    match event {
        BSP_EVENT_SLEEP => {
            sleep_mode_enter();
        }
        BSP_EVENT_DISCONNECT => {
            let err_code = nrf::sd_ble_gap_disconnect(
                M_CONN_HANDLE,
                nrf::BLE_HCI_REMOTE_USER_TERMINATED_CONNECTION as u8,
            );
            if err_code != nrf::NRF_ERROR_INVALID_STATE {
                check(err_code).unwrap();
            }
        }
        BSP_EVENT_WHITELIST_OFF => if M_CONN_HANDLE == nrf::BLE_CONN_HANDLE_INVALID as u16 {
            let err_code = nrf::ble_advertising_restart_without_whitelist();
            if err_code != nrf::NRF_ERROR_INVALID_STATE {
                check(err_code).unwrap();
            }
        },
        _ => {}
    };
}

unsafe extern "C" fn ble_evt_dispatch(p_ble_evt: *mut nrf::ble_evt_t) {
    /** The Connection state module has to be fed BLE events in order to function correctly
     * Remember to call ble_conn_state_on_ble_evt before calling any ble_conns_state_* functions. */
    nrf::ble_conn_state_on_ble_evt(p_ble_evt);
    nrf::pm_on_ble_evt(p_ble_evt);
    nrf::ble_conn_params_on_ble_evt(p_ble_evt);
    nrf::bsp_btn_ble_on_ble_evt(p_ble_evt);
    on_ble_evt(p_ble_evt);
    nrf::ble_advertising_on_ble_evt(p_ble_evt);
    nrf::nrf_ble_gatt_on_ble_evt(&mut M_GATT, p_ble_evt);

    /*YOUR_JOB add calls to _on_ble_evt functions from each service your application is using
       ble_xxs_on_ble_evt(&m_xxs, p_ble_evt);
       ble_yys_on_ble_evt(&m_yys, p_ble_evt);
     */
}

/// Function for dispatching a system event to interested modules.
unsafe extern "C" fn sys_evt_dispatch(evt_id: u32) {
    // Dispatch the system event to the fstorage module, where it will be
    // dispatched to the Flash Data Storage (FDS) module.
    nrf::fs_sys_event_handler(evt_id);

    // Dispatch to the Advertising module last, since it will check if there are any
    // pending flash operations in fstorage. Let fstorage process system events first,
    // so that it can report correctly to the Advertising module.
    nrf::ble_advertising_on_sys_evt(evt_id);
}

unsafe extern "C" fn on_adv_evt(ble_adv_evt: nrf::ble_adv_evt_t) {
    match ble_adv_evt {
        BLE_ADV_EVT_FAST => {
            check(nrf::bsp_indication_set(
                nrf::bsp_indication_t_BSP_INDICATE_ADVERTISING,
            )).unwrap();
        }
        BLE_ADV_EVT_IDLE => {
            sleep_mode_enter();
        }
        _ => {}
    }
}

unsafe extern "C" fn on_conn_params_evt(p_evt: *mut nrf::ble_conn_params_evt_t) {
    match (*p_evt).evt_type {
        BLE_CONN_PARAMS_EVT_FAILED => {
            check(nrf::sd_ble_gap_disconnect(
                M_CONN_HANDLE,
                nrf::BLE_HCI_CONN_INTERVAL_UNACCEPTABLE as u8,
            )).unwrap();
        }
        _ => {}
    }
}

unsafe extern "C" fn conn_params_error_handler(nrf_error: u32) {
    check(nrf_error).unwrap();
}

unsafe fn sleep_mode_enter() {
    check(nrf::bsp_indication_set(
        nrf::bsp_indication_t_BSP_INDICATE_IDLE,
    )).unwrap();

    check(nrf::bsp_btn_ble_sleep_mode_prepare()).unwrap();

    check(nrf::sd_power_system_off()).unwrap();
}

/// Function for starting advertising.
unsafe fn advertising_start(erase_bonds: bool) {
    if erase_bonds {
        delete_bonds();
    // Advertising is started by PM_EVT_PEERS_DELETED_SUCEEDED evetnt
    } else {
        check(nrf::ble_advertising_start(
            nrf::ble_adv_mode_t_BLE_ADV_MODE_FAST,
        )).unwrap();
    }
}

unsafe fn delete_bonds() {
    nrf_log_info("Erase bonds!\r\n\0");

    check(nrf::pm_peers_delete()).unwrap();
}

unsafe extern "C" fn pm_evt_handler(p_evt: *const nrf::pm_evt_t) {
    match (*p_evt).evt_id {
        PM_EVT_BONDED_PEER_CONNECTED => {
            nrf_log_info("Connected to a previously bonded device.\r\n\0");
        }

        PM_EVT_CONN_SEC_SUCCEEDED => {
            nrf_log_info("Connection secured...\r\n\0");
        }

        PM_EVT_CONN_SEC_FAILED => {
            // Often, when securing fails, it shouldn't be restarted, for security reasons.
            // Other times, it can be restarted directly.
            // Sometimes it can be restarted, but only after changing some Security Parameters.
            // Sometimes, it cannot be restarted until the link is disconnected and reconnected.
            // Sometimes it is impossible, to secure the link, or the peer device does not
            // support it.
            // How to handle this error is highly application dependent. */
        }

        PM_EVT_CONN_SEC_CONFIG_REQ => {
            let mut conn_sec_config = nrf::pm_conn_sec_config_t {
                allow_repairing: false,
            };

            nrf::pm_conn_sec_config_reply((*p_evt).conn_handle, &mut conn_sec_config);
        }

        PM_EVT_STORAGE_FULL => {
            let err_code = nrf::fds_gc();

            if (err_code == nrf::FDS_ERR_BUSY as u32)
                || (err_code == nrf::FDS_ERR_NO_SPACE_IN_QUEUES as u32)
            {
                // Retry
            } else {
                check(err_code).unwrap();
            }
        }

        PM_EVT_PEERS_DELETE_SUCCEEDED => {
            advertising_start(false);
        }

        PM_EVT_LOCAL_DB_CACHE_APPLY_FAILED => {
            // The local database has likely changed, send service changed indications.
            nrf::pm_local_database_has_changed();
        }

        PM_EVT_PEER_DATA_UPDATE_FAILED => {
            // Assert.
            // AJM - union read example here!
            check((*p_evt).params.peer_data_update_failed.error).unwrap();
        }

        PM_EVT_PEER_DELETE_FAILED => {
            // Assert.
            check((*p_evt).params.peer_delete_failed.error).unwrap();
        }

        PM_EVT_PEERS_DELETE_FAILED => {
            // Assert.
            check((*p_evt).params.peers_delete_failed_evt.error).unwrap();
        }

        PM_EVT_ERROR_UNEXPECTED => {
            // Assert.
            check((*p_evt).params.error_unexpected.error).unwrap();
        }

        _ => {}
    }
}

/// Function for handling the Application's BLE Stack events.
unsafe fn on_ble_evt(p_ble_evt: *mut nrf::ble_evt_t) {
    let x = (*p_ble_evt).header.evt_id;

    // We can't use a match here because the nordic mixes enum types :(
    if x == nrf::BLE_GAP_EVTS_BLE_GAP_EVT_DISCONNECTED as u16 {
        nrf_log_info("Disconnected.\r\n\0");

        check(nrf::bsp_indication_set(
            nrf::bsp_indication_t_BSP_INDICATE_IDLE,
        )).unwrap();
    } else if x == nrf::BLE_GAP_EVTS_BLE_GAP_EVT_CONNECTED as u16 {
        nrf_log_info("Connected.\r\n\0");
        check(nrf::bsp_indication_set(
            nrf::bsp_indication_t_BSP_INDICATE_CONNECTED,
        )).unwrap();
        M_CONN_HANDLE = (*p_ble_evt).evt.gap_evt.conn_handle;
    } else if x == nrf::BLE_GATTC_EVTS_BLE_GATTC_EVT_TIMEOUT as u16 {
        nrf_log_info("GATT Client Timeout.\r\n\0");
        check(nrf::sd_ble_gap_disconnect(
            (*p_ble_evt).evt.gattc_evt.conn_handle,
            nrf::BLE_HCI_REMOTE_USER_TERMINATED_CONNECTION as u8,
        )).unwrap();
    } else if x == nrf::BLE_GATTC_EVTS_BLE_GATTC_EVT_TIMEOUT as u16 {
        nrf_log_info("GATT Client Timeout.\r\n\0");
        check(nrf::sd_ble_gap_disconnect(
            (*p_ble_evt).evt.gattc_evt.conn_handle,
            nrf::BLE_HCI_REMOTE_USER_TERMINATED_CONNECTION as u8,
        )).unwrap();
    } else if x == nrf::BLE_GATTS_EVTS_BLE_GATTS_EVT_TIMEOUT as u16 {
        nrf_log_info("GATT Server Timeout.\r\n\0");
        check(nrf::sd_ble_gap_disconnect(
            (*p_ble_evt).evt.gatts_evt.conn_handle,
            nrf::BLE_HCI_REMOTE_USER_TERMINATED_CONNECTION as u8,
        )).unwrap();
    } else if x == nrf::BLE_COMMON_EVTS_BLE_EVT_USER_MEM_REQUEST as u16 {
        check(nrf::sd_ble_user_mem_reply(
            (*p_ble_evt).evt.gattc_evt.conn_handle,
            core::ptr::null(),
        )).unwrap();
    } else if x == nrf::BLE_GATTS_EVTS_BLE_GATTS_EVT_RW_AUTHORIZE_REQUEST as u16 {
        let req = (*p_ble_evt).evt.gatts_evt.params.authorize_request;

        if req.type_ != nrf::BLE_GATTS_AUTHORIZE_TYPE_INVALID as u8 {
            let op = req.request.write.op;
            if (op == nrf::BLE_GATTS_OP_PREP_WRITE_REQ as u8)
                || (op == nrf::BLE_GATTS_OP_EXEC_WRITE_REQ_NOW as u8)
                || (op == nrf::BLE_GATTS_OP_EXEC_WRITE_REQ_CANCEL as u8)
            {
                let mut auth_reply = nrf::ble_gatts_rw_authorize_reply_params_t::default();

                auth_reply.type_ = if req.type_ == nrf::BLE_GATTS_AUTHORIZE_TYPE_WRITE as u8 {
                    nrf::BLE_GATTS_AUTHORIZE_TYPE_WRITE as u8
                } else {
                    nrf::BLE_GATTS_AUTHORIZE_TYPE_READ as u8
                };

                auth_reply.params.write.gatt_status = APP_FEATURE_NOT_SUPPORTED;

                check(nrf::sd_ble_gatts_rw_authorize_reply(
                    (*p_ble_evt).evt.gatts_evt.conn_handle,
                    &auth_reply,
                )).unwrap();
            }
        }
    } else {
        // No implementation needed.
    }
}
