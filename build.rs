extern crate gcc;

use std::env;
use std::path::PathBuf;

use std::process::Command;

use std::fs::File;
use std::io::Write;

use gcc::Config;

fn main() {

    let outdir = env::var("OUT_DIR").unwrap();

    // If any of these files/folders change, we should regenerate
    //   the whole C + bindings component
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rerun-if-changed=memory.x");

    // TODO: glob all contents of folders, as cargo doesn't traverse
    println!("cargo:rerun-if-changed=nRF5-sdk");
    println!("cargo:rerun-if-changed=shims");

    process_linker_file(&outdir);
    generate_ble(&outdir);
    make_c_deps(&outdir);
}

fn process_linker_file(outdir: &String) {
    let out = &PathBuf::from(outdir);

    // Copy over the target specific linker script
    File::create(out.join("nrf52dk-sys.ld"))
        .unwrap()
        .write_all(include_bytes!("nrf52dk-sys.ld"))
        .unwrap();

    // Also copy the nrf general linker script
    File::create(out.join("nrf5x_common.ld"))
        .unwrap()
        .write_all(include_bytes!("nrf5x_common.ld"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
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

    // Then the rest
    for i in INCLUDE_PATHS {
        config.include(i);
    }

    config.compile("libnrf.a");

    println!("cargo:rustc-link-search={}", outdir);
    println!("cargo:rustc-link-lib=static=nrf");
}

fn generate_ble(outdir: &String) {
    let out = &PathBuf::from(outdir);
    let out2 = out.join("bindings.rs");
    let out3 = out2.to_string_lossy();


    // TODO version assert

    let mut cmd = Command::new("bindgen");

    // Bindgen Opts
    cmd.arg("bindings.h");
    cmd.arg("--no-doc-comments");
    cmd.arg("--use-core");
    cmd.arg("--ctypes-prefix=ctypes");
    cmd.arg("--no-unstable-rust");
    cmd.arg("--with-derive-default");

    cmd.arg("--output");
    cmd.arg(out3.as_ref());

    // Clang Opts begin here
    cmd.arg("--");

    // Standard defines (common with GCC build)
    for &(var, oval) in DEFINES {
        match oval {
            None => cmd.arg(format!("-D{}", var)),
            Some(val) => cmd.arg(format!("-D{}={}", var, val)),
        };
    }

    // Hack defines to make bindgen work
    cmd.arg("-D__CMSIS_GCC_H");
    cmd.arg("-DSVCALL_AS_NORMAL_FUNCTION"); // this is questionable

    // Standard include paths (common with GCC build)
    for inc in INCLUDE_PATHS {
        cmd.arg(format!("-I{}", inc));
    }

    // Final Clang args
    cmd.arg("-fshort-enums");
    cmd.arg("-target");
    cmd.arg(env::var("TARGET").unwrap());

    assert!(cmd.status()
                .expect("failed to build Blue libs")
                .success());
}


static FILES: &[&str] = &["./shims/shimmy.c",

                          "./nRF5-sdk/components/toolchain/gcc/gcc_startup_nrf52.S",

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
                          "./nRF5-sdk/external/segger_rtt/SEGGER_RTT_printf.c"];

static INCLUDE_PATHS: &[&str] = &["./shims", // FIXME sdk_config.h shouldn't be hardcoded

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
                                  "./nRF5-sdk/external/segger_rtt"];

static FLAGS: &[&str] = &["-std=c99",
                          "-mcpu=cortex-m4",
                          "-mthumb",
                          "-mabi=aapcs",
                          "-mfloat-abi=hard",
                          "-mfpu=fpv4-sp-d16",
                          "-ffunction-sections",
                          "-fdata-sections",
                          "-fno-strict-aliasing",
                          "-fno-builtin",
                          "--short-enums"];

static DEFINES: &[(&str, Option<&str>)] = &[("BLE_STACK_SUPPORT_REQD", None),
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
                                            ("SWI_DISABLE0", None)];
