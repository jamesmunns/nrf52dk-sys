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
#include "ble.h"
#include "app_timer.h"