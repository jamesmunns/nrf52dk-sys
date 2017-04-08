extern crate bindgen;
extern crate glob;

use std::env;
use std::path::PathBuf;
use bindgen::Builder;

use std::process::Command;
use glob::glob;

use std::fs::File;
use std::io::Write;

fn main() {

    let outdir = env::var("OUT_DIR").unwrap();

    // NOTE: If we get linking errors later, these might be needed somehow:
    // println!("cargo:rustc-link-search=native={}", outdir);
    // println!("cargo:libdir=???");

    // Okay, these build times are reeeeeeally long. I need a smarter way
    //   to do this, but for now this should be fine. If you need to force
    //   rebuild, touch build.rs.
    println!("cargo:rerun-if-changed=build.rs");

    process_map_file(&outdir);
    generate_ble(&outdir);
    make_c_deps(&outdir);
}

fn process_map_file(outdir: &String) {
    let out = &PathBuf::from(outdir);
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();


    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}

fn make_c_deps(outdir: &String) {
    assert!(Command::new("make")
        .arg("-C")
        .arg("shims")
        .arg("-j8") // TODO
        .status()
        .expect("failed to build Blue libs")
        .success());

    let out_path = PathBuf::from(outdir);

    assert!(Command::new("arm-none-eabi-ar")
        .arg("crus")
        .arg(out_path.join("libnrf.a"))
        .args(&glob("./shims/_build/*.o")
            .expect("Failed to read glob pattern")
            .filter_map(|x| x.ok())
            .collect::<Vec<PathBuf>>())
        .status()
        .expect("failed to create blue archive")
        .success());

    println!("cargo:rustc-link-search={}", outdir);
    println!("cargo:rustc-link-lib=static=nrf");
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
        .clang_arg("-I./nRF5-sdk/components")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_advertising")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_dtm")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_racp")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_ancs_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_ans_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_bas")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_bas_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_cscs")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_cts_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_dfu")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_dis")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_gls")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_hids")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_hrs")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_hrs_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_hts")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_ias")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_ias_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_lbs")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_lbs_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_lls")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_nus")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_nus_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_rscs")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_rscs_c")
        .clang_arg("-I./nRF5-sdk/components/ble/ble_services/ble_tps")
        .clang_arg("-I./nRF5-sdk/components/ble/common")
        .clang_arg("-I./nRF5-sdk/components/ble/nrf_ble_gatt")
        .clang_arg("-I./nRF5-sdk/components/ble/nrf_ble_qwr")
        .clang_arg("-I./nRF5-sdk/components/ble/peer_manager")
        .clang_arg("-I./nRF5-sdk/components/boards")
        .clang_arg("-I./nRF5-sdk/components/device")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/clock")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/common")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/comp")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/delay")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/gpiote")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/hal")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/i2s")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/lpcomp")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/pdm")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/power")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/ppi")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/pwm")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/qdec")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/rng")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/rtc")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/saadc")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/spi_master")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/spi_slave")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/swi")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/timer")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/twi_master")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/twis_slave")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/uart")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/usbd")
        .clang_arg("-I./nRF5-sdk/components/drivers_nrf/wdt")
        .clang_arg("-I./nRF5-sdk/components/libraries/bsp")
        .clang_arg("-I./nRF5-sdk/components/libraries/button")
        .clang_arg("-I./nRF5-sdk/components/libraries/crc16")
        .clang_arg("-I./nRF5-sdk/components/libraries/crc32")
        .clang_arg("-I./nRF5-sdk/components/libraries/csense")
        .clang_arg("-I./nRF5-sdk/components/libraries/csense_drv")
        .clang_arg("-I./nRF5-sdk/components/libraries/ecc")
        .clang_arg("-I./nRF5-sdk/components/libraries/experimental_section_vars")
        .clang_arg("-I./nRF5-sdk/components/libraries/fds")
        .clang_arg("-I./nRF5-sdk/components/libraries/fstorage")
        .clang_arg("-I./nRF5-sdk/components/libraries/gpiote")
        .clang_arg("-I./nRF5-sdk/components/libraries/hardfault")
        .clang_arg("-I./nRF5-sdk/components/libraries/hci")
        .clang_arg("-I./nRF5-sdk/components/libraries/led_softblink")
        .clang_arg("-I./nRF5-sdk/components/libraries/log")
        .clang_arg("-I./nRF5-sdk/components/libraries/log/src")
        .clang_arg("-I./nRF5-sdk/components/libraries/low_power_pwm")
        .clang_arg("-I./nRF5-sdk/components/libraries/mem_manager")
        .clang_arg("-I./nRF5-sdk/components/libraries/pwm")
        .clang_arg("-I./nRF5-sdk/components/libraries/queue")
        .clang_arg("-I./nRF5-sdk/components/libraries/scheduler")
        .clang_arg("-I./nRF5-sdk/components/libraries/sensorsim")
        .clang_arg("-I./nRF5-sdk/components/libraries/slip")
        .clang_arg("-I./nRF5-sdk/components/libraries/strerror")
        .clang_arg("-I./nRF5-sdk/components/libraries/timer")
        .clang_arg("-I./nRF5-sdk/components/libraries/twi")
        .clang_arg("-I./nRF5-sdk/components/libraries/uart")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/audio")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/cdc")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/cdc/acm")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/hid")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/hid/generic")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/hid/kbd")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/hid/mouse")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/class/msc")
        .clang_arg("-I./nRF5-sdk/components/libraries/usbd/config")
        .clang_arg("-I./nRF5-sdk/components/libraries/util")
        .clang_arg("-I./nRF5-sdk/components/softdevice/common/softdevice_handler")
        .clang_arg("-I./nRF5-sdk/components/softdevice/s132/headers")
        .clang_arg("-I./nRF5-sdk/components/softdevice/s132/headers/nrf52")
        .clang_arg("-I./nRF5-sdk/components/toolchain")
        .clang_arg("-I./nRF5-sdk/components/toolchain/cmsis/include")
        .clang_arg("-I./nRF5-sdk/components/toolchain/gcc")
        .clang_arg("-I./nRF5-sdk/external/segger_rtt")

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

