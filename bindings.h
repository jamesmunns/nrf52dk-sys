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
#include "app_error.h"
#include "app_timer.h"
#include "ble.h"
#include "ble_advdata.h"
#include "ble_advertising.h"
#include "ble_conn_params.h"
#include "ble_conn_state.h"
#include "ble_hci.h"
#include "ble_srv_common.h"
#include "boards.h"
#include "bsp_btn_ble.h"
#include "fds.h"
#include "fstorage.h"
#include "id_manager.h"
#include "nordic_common.h"
#include "nrf.h"
#include "nrf_ble_gatt.h"
#include "nrf_gpio.h"
#include "nrf_log.h"
#include "nrf_log_ctrl.h"
#include "peer_manager.h"
#include "sensorsim.h"
#include "softdevice_handler.h"

// We have to go deeper
#include "ble_stack_handler_types.h"