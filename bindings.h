// Make sure that nrf doesn't think we are compiling for a PC
#if defined(_WIN32)
    #undef _WIN32
#endif
#if defined(__unix)
    #undef __unix
#endif
#if defined(__APPLE__)
    #undef __APPLE__
#endif

// Generate bindings for these files
//   These items were pulled from the template Makefile, in this order
#include "nordic_common.h"
#include "nrf.h"
#include "app_error.h"
#include "ble.h"
#include "ble_hci.h"
#include "ble_srv_common.h"
#include "ble_advdata.h"
#include "ble_advertising.h"
#include "ble_conn_params.h"
#include "boards.h"
#include "softdevice_handler.h"
#include "app_timer.h"
#include "fstorage.h"
#include "fds.h"
#include "peer_manager.h"
#include "bsp_btn_ble.h"
#include "sensorsim.h"
#include "nrf_gpio.h"
#include "ble_conn_state.h"
#include "nrf_ble_gatt.h"

// These items were missing, but are probably useful/needed
#include "nrf_log_ctrl.h"