#pragma once

/* nrf_delay shim */
#include "nrf_sdm.h"

nrf_clock_lf_cfg_t _NRF_CLOCK_LFCLKSRC();
void _nrf_delay_ms(uint32_t number_of_ms);
