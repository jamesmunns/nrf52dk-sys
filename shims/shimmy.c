/* NOTE:
 *
 * This include forces the SVCALL macro to be rendered and compiled here.
 * This works for now, but we should probably switch to a rust macro similar
 * to how ble400 handles SVCALL.
 */
#include "nrf_svc.h"
#include "../bindings.h"

/* NOTE:
 *
 * This section renders the static inline nrf_delay_(us|ms) so it is
 * usable with Rust
 */
#include "nrf_delay.h"
#include "nrf_sdm.h"
#include "shims.h"

void _nrf_delay_ms(uint32_t number_of_ms) { nrf_delay_ms(number_of_ms); }

nrf_clock_lf_cfg_t _NRF_CLOCK_LFCLKSRC() {
  return (nrf_clock_lf_cfg_t)NRF_CLOCK_LFCLKSRC;
}
