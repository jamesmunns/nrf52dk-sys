extern crate bindgen;
extern crate glob;
extern crate gcc;

use std::env;
use std::path::PathBuf;
use bindgen::Builder;

use std::process::Command;
use glob::glob;

use std::fs::File;
use std::io::Write;

use gcc::Config;

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
    let mut config = Config::new();
    let out_path = PathBuf::from(outdir);

    config.out_dir(out_path);

    for f in FLAGS {
        config.flag(f);
    }

    for &(var, val) in DEFINES {
        config.define(var, val);
    }

    for f in FILES {
        config.file(f);
    }

    // FIXME sdk_config.h shouldn't be hardcoded
    config.include("./shims");

    // Then the rest
    for i in INCLUDE_PATHS {
        config.include(i);
    }

    config.compile("libnrf.a");

    println!("cargo:rustc-link-search={}", outdir);
    println!("cargo:rustc-link-lib=static=nrf");
}

fn generate_ble(outdir: &String) {
    // panic!(env::var("TARGET").unwrap());

    let bindings = Builder::default()
        .no_unstable_rust()
        .use_core()
        .generate_inline_functions(true)
        .ctypes_prefix("ctypes")

        .header("bindings.h")

        // Defines
        .clang_arg("-DSVCALL_AS_NORMAL_FUNCTION") // this is questionable
        // .clang_arg("-target")
        // .clang_arg("thumbv7em-none-eabihf")

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

        .clang_arg("-fshort-enums") // grr

        .clang_arg("-D__CMSIS_GCC_H")

        // More structure alignment - not sure if either/both of these are necessary
        // .clang_arg("-fpack-struct=4")
        // .clang_arg("-fmax-type-align=4")
        // .clang_arg("-m32")

        .clang_arg(format!("--target={}", env::var("TARGET").unwrap()))
        .clang_arg("-mcpu=cortex-m4")

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





static FILES: &[&str] = &[
    "./shims/shimmy.c",

    "./nRF5-sdk/components/ble/ble_advertising/ble_advertising.c",
    "./nRF5-sdk/components/ble/common/ble_advdata.c",
    "./nRF5-sdk/components/ble/common/ble_conn_params.c",
    "./nRF5-sdk/components/ble/common/ble_conn_state.c",
    "./nRF5-sdk/components/ble/common/ble_srv_common.c",
    "./nRF5-sdk/components/ble/nrf_ble_gatt/nrf_ble_gatt.c",
    "./nRF5-sdk/components/ble/peer_manager/gatt_cache_manager.c",
    "./nRF5-sdk/components/ble/peer_manager/gatts_cache_manager.c",
    "./nRF5-sdk/components/ble/peer_manager/id_manager.c",
    "./nRF5-sdk/components/ble/peer_manager/peer_data_storage.c",
    "./nRF5-sdk/components/ble/peer_manager/peer_database.c",
    "./nRF5-sdk/components/ble/peer_manager/peer_id.c",
    "./nRF5-sdk/components/ble/peer_manager/peer_manager.c",
    "./nRF5-sdk/components/ble/peer_manager/pm_buffer.c",
    "./nRF5-sdk/components/ble/peer_manager/pm_mutex.c",
    "./nRF5-sdk/components/ble/peer_manager/security_dispatcher.c",
    "./nRF5-sdk/components/ble/peer_manager/security_manager.c",
    "./nRF5-sdk/components/boards/boards.c",
    "./nRF5-sdk/components/drivers_nrf/clock/nrf_drv_clock.c",
    "./nRF5-sdk/components/drivers_nrf/common/nrf_drv_common.c",
    "./nRF5-sdk/components/drivers_nrf/gpiote/nrf_drv_gpiote.c",
    "./nRF5-sdk/components/drivers_nrf/uart/nrf_drv_uart.c",
    "./nRF5-sdk/components/libraries/bsp/bsp.c",
    "./nRF5-sdk/components/libraries/bsp/bsp_btn_ble.c",
    "./nRF5-sdk/components/libraries/bsp/bsp_nfc.c",
    "./nRF5-sdk/components/libraries/button/app_button.c",
    "./nRF5-sdk/components/libraries/crc16/crc16.c",
    "./nRF5-sdk/components/libraries/fds/fds.c",
    "./nRF5-sdk/components/libraries/fstorage/fstorage.c",
    "./nRF5-sdk/components/libraries/hardfault/hardfault_implementation.c",
    "./nRF5-sdk/components/libraries/log/src/nrf_log_backend_serial.c",
    "./nRF5-sdk/components/libraries/log/src/nrf_log_frontend.c",
    "./nRF5-sdk/components/libraries/scheduler/app_scheduler.c",
    "./nRF5-sdk/components/libraries/sensorsim/sensorsim.c",
    "./nRF5-sdk/components/libraries/strerror/nrf_strerror.c",
    "./nRF5-sdk/components/libraries/timer/app_timer.c",
    "./nRF5-sdk/components/libraries/util/app_error.c",
    "./nRF5-sdk/components/libraries/util/app_error_weak.c",
    "./nRF5-sdk/components/libraries/util/app_util_platform.c",
    "./nRF5-sdk/components/libraries/util/nrf_assert.c",
    "./nRF5-sdk/components/libraries/util/sdk_mapped_flags.c",
    "./nRF5-sdk/components/softdevice/common/softdevice_handler/softdevice_handler.c",
    "./nRF5-sdk/components/toolchain/system_nrf52.c",
    "./nRF5-sdk/external/segger_rtt/RTT_Syscalls_GCC.c",
    "./nRF5-sdk/external/segger_rtt/SEGGER_RTT.c",
    "./nRF5-sdk/external/segger_rtt/SEGGER_RTT_printf.c",
];

static INCLUDE_PATHS: &[&str] = &[
    "./nRF5-sdk/components",
    "./nRF5-sdk/components/ble/ble_advertising",
    "./nRF5-sdk/components/ble/ble_dtm",
    "./nRF5-sdk/components/ble/ble_racp",
    "./nRF5-sdk/components/ble/ble_services/ble_ancs_c",
    "./nRF5-sdk/components/ble/ble_services/ble_ans_c",
    "./nRF5-sdk/components/ble/ble_services/ble_bas",
    "./nRF5-sdk/components/ble/ble_services/ble_bas_c",
    "./nRF5-sdk/components/ble/ble_services/ble_cscs",
    "./nRF5-sdk/components/ble/ble_services/ble_cts_c",
    "./nRF5-sdk/components/ble/ble_services/ble_dfu",
    "./nRF5-sdk/components/ble/ble_services/ble_dis",
    "./nRF5-sdk/components/ble/ble_services/ble_gls",
    "./nRF5-sdk/components/ble/ble_services/ble_hids",
    "./nRF5-sdk/components/ble/ble_services/ble_hrs",
    "./nRF5-sdk/components/ble/ble_services/ble_hrs_c",
    "./nRF5-sdk/components/ble/ble_services/ble_hts",
    "./nRF5-sdk/components/ble/ble_services/ble_ias",
    "./nRF5-sdk/components/ble/ble_services/ble_ias_c",
    "./nRF5-sdk/components/ble/ble_services/ble_lbs",
    "./nRF5-sdk/components/ble/ble_services/ble_lbs_c",
    "./nRF5-sdk/components/ble/ble_services/ble_lls",
    "./nRF5-sdk/components/ble/ble_services/ble_nus",
    "./nRF5-sdk/components/ble/ble_services/ble_nus_c",
    "./nRF5-sdk/components/ble/ble_services/ble_rscs",
    "./nRF5-sdk/components/ble/ble_services/ble_rscs_c",
    "./nRF5-sdk/components/ble/ble_services/ble_tps",
    "./nRF5-sdk/components/ble/common",
    "./nRF5-sdk/components/ble/nrf_ble_gatt",
    "./nRF5-sdk/components/ble/nrf_ble_qwr",
    "./nRF5-sdk/components/ble/peer_manager",
    "./nRF5-sdk/components/boards",
    "./nRF5-sdk/components/device",
    "./nRF5-sdk/components/drivers_nrf/clock",
    "./nRF5-sdk/components/drivers_nrf/common",
    "./nRF5-sdk/components/drivers_nrf/comp",
    "./nRF5-sdk/components/drivers_nrf/delay",
    "./nRF5-sdk/components/drivers_nrf/gpiote",
    "./nRF5-sdk/components/drivers_nrf/hal",
    "./nRF5-sdk/components/drivers_nrf/i2s",
    "./nRF5-sdk/components/drivers_nrf/lpcomp",
    "./nRF5-sdk/components/drivers_nrf/pdm",
    "./nRF5-sdk/components/drivers_nrf/power",
    "./nRF5-sdk/components/drivers_nrf/ppi",
    "./nRF5-sdk/components/drivers_nrf/pwm",
    "./nRF5-sdk/components/drivers_nrf/qdec",
    "./nRF5-sdk/components/drivers_nrf/rng",
    "./nRF5-sdk/components/drivers_nrf/rtc",
    "./nRF5-sdk/components/drivers_nrf/saadc",
    "./nRF5-sdk/components/drivers_nrf/spi_master",
    "./nRF5-sdk/components/drivers_nrf/spi_slave",
    "./nRF5-sdk/components/drivers_nrf/swi",
    "./nRF5-sdk/components/drivers_nrf/timer",
    "./nRF5-sdk/components/drivers_nrf/twi_master",
    "./nRF5-sdk/components/drivers_nrf/twis_slave",
    "./nRF5-sdk/components/drivers_nrf/uart",
    "./nRF5-sdk/components/drivers_nrf/usbd",
    "./nRF5-sdk/components/drivers_nrf/wdt",
    "./nRF5-sdk/components/libraries/bsp",
    "./nRF5-sdk/components/libraries/button",
    "./nRF5-sdk/components/libraries/crc16",
    "./nRF5-sdk/components/libraries/crc32",
    "./nRF5-sdk/components/libraries/csense",
    "./nRF5-sdk/components/libraries/csense_drv",
    "./nRF5-sdk/components/libraries/ecc",
    "./nRF5-sdk/components/libraries/experimental_section_vars",
    "./nRF5-sdk/components/libraries/fds",
    "./nRF5-sdk/components/libraries/fstorage",
    "./nRF5-sdk/components/libraries/gpiote",
    "./nRF5-sdk/components/libraries/hardfault",
    "./nRF5-sdk/components/libraries/hci",
    "./nRF5-sdk/components/libraries/led_softblink",
    "./nRF5-sdk/components/libraries/log",
    "./nRF5-sdk/components/libraries/log/src",
    "./nRF5-sdk/components/libraries/low_power_pwm",
    "./nRF5-sdk/components/libraries/mem_manager",
    "./nRF5-sdk/components/libraries/pwm",
    "./nRF5-sdk/components/libraries/queue",
    "./nRF5-sdk/components/libraries/scheduler",
    "./nRF5-sdk/components/libraries/sensorsim",
    "./nRF5-sdk/components/libraries/slip",
    "./nRF5-sdk/components/libraries/strerror",
    "./nRF5-sdk/components/libraries/timer",
    "./nRF5-sdk/components/libraries/twi",
    "./nRF5-sdk/components/libraries/uart",
    "./nRF5-sdk/components/libraries/usbd",
    "./nRF5-sdk/components/libraries/usbd/class/audio",
    "./nRF5-sdk/components/libraries/usbd/class/cdc",
    "./nRF5-sdk/components/libraries/usbd/class/cdc/acm",
    "./nRF5-sdk/components/libraries/usbd/class/hid",
    "./nRF5-sdk/components/libraries/usbd/class/hid/generic",
    "./nRF5-sdk/components/libraries/usbd/class/hid/kbd",
    "./nRF5-sdk/components/libraries/usbd/class/hid/mouse",
    "./nRF5-sdk/components/libraries/usbd/class/msc",
    "./nRF5-sdk/components/libraries/usbd/config",
    "./nRF5-sdk/components/libraries/util",
    "./nRF5-sdk/components/softdevice/common/softdevice_handler",
    "./nRF5-sdk/components/softdevice/s132/headers",
    "./nRF5-sdk/components/softdevice/s132/headers/nrf52",
    "./nRF5-sdk/components/toolchain",
    "./nRF5-sdk/components/toolchain/cmsis/include",
    "./nRF5-sdk/components/toolchain/gcc",
    "./nRF5-sdk/external/segger_rtt",
];

static FLAGS: &[&str] = &[
    "-std=c99",
    "-mcpu=cortex-m4", //
    "-mthumb", //
    "-mabi=aapcs", //
    "-mfloat-abi=hard", //
    "-mfpu=fpv4-sp-d16",
    "-ffunction-sections", //
    "-fdata-sections", //
    "-fno-strict-aliasing", //
    "-fno-builtin", //
    "--short-enums", //

    // "-Wall",
    // "-Werror",
    // "-O3",
];

static DEFINES: &[(&str, Option<&str>)] = &[
    ("BLE_STACK_SUPPORT_REQD", None),
    ("BOARD_PCA10040", None),
    ("CONFIG_GPIO_AS_PINRESET", None),
    ("NRF52", None),
    ("NRF52832_XXAA", None),
    ("NRF52_PAN_12", None),
    ("NRF52_PAN_15", None),
    ("NRF52_PAN_20", None),
    ("NRF52_PAN_31", None),
    ("NRF52_PAN_36", None),
    ("NRF52_PAN_51", None),
    ("NRF52_PAN_54", None),
    ("NRF52_PAN_55", None),
    ("NRF52_PAN_58", None),
    ("NRF52_PAN_64", None),
    ("NRF52_PAN_74", None),
    ("NRF_SD_BLE_API_VERSION", Some("4")),
    ("S132", None),
    ("SOFTDEVICE_PRESENT", None),
    ("SWI_DISABLE0", None),
];