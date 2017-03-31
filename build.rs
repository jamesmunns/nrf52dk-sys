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
        .clang_arg("-DNRF52832_XXAA")
        .clang_arg("-DSVCALL_AS_NORMAL_FUNCTION") // this is questionable

        // sdk_config.h - TODO
        .clang_arg("-I.")

        // Primary dependencies
        .clang_arg("-I../nrf5_sdk/components/libraries/timer/")
        .clang_arg("-I../nrf5_sdk/components/softdevice/s132/headers")

        // Secondary dependencies
        .clang_arg("-I../nrf5_sdk/components/libraries/util/")
        .clang_arg("-I../nrf5_sdk/components/device")
        .clang_arg("-I../nrf5_sdk/components/toolchain")
        .clang_arg("-I../nrf5_sdk/components/toolchain/cmsis/include")

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

