extern crate bindgen;
// extern crate gcc;

// use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
// use std::sync::{Arc, RwLock};
use bindgen::Builder;
// use bindgen::callbacks::ParseCallbacks;

// #[derive(Debug)]
// struct MacroCallback {
//     macros: Arc<RwLock<HashSet<String>>>,
// }

// impl ParseCallbacks for MacroCallback {
//     fn parsed_macro(&self, _name: &str) {
//         self.macros.write().unwrap().insert(String::from(_name));
//     }
// }

fn main() {
    // gcc::Config::new()
    //     .cpp(true)
    //     .file("cpp/Test.cc")
    //     .compile("libtest.a");

    // let macros = Arc::new(RwLock::new(HashSet::new()));
    let outdir = env::var("OUT_DIR").unwrap();

    generate_ble(&outdir);
}

fn generate_ble(outdir: &String) {
    let bindings = Builder::default()
        .no_unstable_rust()
        .use_core()
        .generate_inline_functions(true)
        .ctypes_prefix("ctypes")

        .header("bindings.h")

        // Defines
        .clang_arg("-DSVCALL_AS_NORMAL_FUNCTION") // this is questionable

        // sdk_config.h - TODO
        .clang_arg("-I.")

        // Welp. Includes.
        .clang_arg("-I../nrf5_sdk/components")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_advertising")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_dtm")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_racp")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_ancs_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_ans_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_bas")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_bas_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_cscs")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_cts_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_dfu")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_dis")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_gls")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_hids")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_hrs")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_hrs_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_hts")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_ias")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_ias_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_lbs")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_lbs_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_lls")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_nus")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_nus_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_rscs")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_rscs_c")
        .clang_arg("-I../nrf5_sdk/components/ble/ble_services/ble_tps")
        .clang_arg("-I../nrf5_sdk/components/ble/common")
        .clang_arg("-I../nrf5_sdk/components/ble/nrf_ble_gatt")
        .clang_arg("-I../nrf5_sdk/components/ble/nrf_ble_qwr")
        .clang_arg("-I../nrf5_sdk/components/ble/peer_manager")
        .clang_arg("-I../nrf5_sdk/components/boards")
        .clang_arg("-I../nrf5_sdk/components/device")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/clock")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/common")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/comp")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/delay")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/gpiote")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/hal")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/i2s")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/lpcomp")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/pdm")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/power")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/ppi")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/pwm")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/qdec")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/rng")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/rtc")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/saadc")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/spi_master")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/spi_slave")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/swi")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/timer")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/twi_master")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/twis_slave")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/uart")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/usbd")
        .clang_arg("-I../nrf5_sdk/components/drivers_nrf/wdt")
        .clang_arg("-I../nrf5_sdk/components/libraries/bsp")
        .clang_arg("-I../nrf5_sdk/components/libraries/button")
        .clang_arg("-I../nrf5_sdk/components/libraries/crc16")
        .clang_arg("-I../nrf5_sdk/components/libraries/crc32")
        .clang_arg("-I../nrf5_sdk/components/libraries/csense")
        .clang_arg("-I../nrf5_sdk/components/libraries/csense_drv")
        .clang_arg("-I../nrf5_sdk/components/libraries/ecc")
        .clang_arg("-I../nrf5_sdk/components/libraries/experimental_section_vars")
        .clang_arg("-I../nrf5_sdk/components/libraries/fds")
        .clang_arg("-I../nrf5_sdk/components/libraries/fstorage")
        .clang_arg("-I../nrf5_sdk/components/libraries/gpiote")
        .clang_arg("-I../nrf5_sdk/components/libraries/hardfault")
        .clang_arg("-I../nrf5_sdk/components/libraries/hci")
        .clang_arg("-I../nrf5_sdk/components/libraries/led_softblink")
        .clang_arg("-I../nrf5_sdk/components/libraries/log")
        .clang_arg("-I../nrf5_sdk/components/libraries/log/src")
        .clang_arg("-I../nrf5_sdk/components/libraries/low_power_pwm")
        .clang_arg("-I../nrf5_sdk/components/libraries/mem_manager")
        .clang_arg("-I../nrf5_sdk/components/libraries/pwm")
        .clang_arg("-I../nrf5_sdk/components/libraries/queue")
        .clang_arg("-I../nrf5_sdk/components/libraries/scheduler")
        .clang_arg("-I../nrf5_sdk/components/libraries/sensorsim")
        .clang_arg("-I../nrf5_sdk/components/libraries/slip")
        .clang_arg("-I../nrf5_sdk/components/libraries/strerror")
        .clang_arg("-I../nrf5_sdk/components/libraries/timer")
        .clang_arg("-I../nrf5_sdk/components/libraries/twi")
        .clang_arg("-I../nrf5_sdk/components/libraries/uart")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/audio")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/cdc")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/cdc/acm")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/hid")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/hid/generic")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/hid/kbd")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/hid/mouse")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/class/msc")
        .clang_arg("-I../nrf5_sdk/components/libraries/usbd/config")
        .clang_arg("-I../nrf5_sdk/components/libraries/util")
        .clang_arg("-I../nrf5_sdk/components/softdevice/common/softdevice_handler")
        .clang_arg("-I../nrf5_sdk/components/softdevice/s132/headers")
        .clang_arg("-I../nrf5_sdk/components/softdevice/s132/headers/nrf52")
        .clang_arg("-I../nrf5_sdk/components/toolchain")
        .clang_arg("-I../nrf5_sdk/components/toolchain/cmsis/include")
        .clang_arg("-I../nrf5_sdk/components/toolchain/gcc")
        .clang_arg("-I../nrf5_sdk/external/segger_rtt")

        // Welp. Defines
        .clang_arg("-DBLE_STACK_SUPPORT_REQD")
        .clang_arg("-DBOARD_PCA10040") // TODO - this probably needs to change
        .clang_arg("-DCONFIG_GPIO_AS_PINRESET")
        .clang_arg("-DNRF52")
        .clang_arg("-DNRF52832_XXAA")
        .clang_arg("-DNRF52_PAN_12")
        .clang_arg("-DNRF52_PAN_15")
        .clang_arg("-DNRF52_PAN_20")
        .clang_arg("-DNRF52_PAN_31")
        .clang_arg("-DNRF52_PAN_36")
        .clang_arg("-DNRF52_PAN_51")
        .clang_arg("-DNRF52_PAN_54")
        .clang_arg("-DNRF52_PAN_55")
        .clang_arg("-DNRF52_PAN_58")
        .clang_arg("-DNRF52_PAN_64")
        .clang_arg("-DNRF52_PAN_74")
        .clang_arg("-DNRF_SD_BLE_API_VERSION=4")
        .clang_arg("-DS132")
        .clang_arg("-DSOFTDEVICE_PRESENT")
        .clang_arg("-DSWI_DISABLE0")

        .clang_arg(env::var("TARGET").unwrap())

        // some of the core.h doxygen comments fuck up the parser
        //   tracking issue: https://github.com/servo/rust-bindgen/issues/426
        .generate_comments(false)

        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(outdir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write Nordic bindings!");

    println!("cargo:rustc-link-search=native={}", outdir);
}

