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

    let bindings = Builder::default()
        .no_unstable_rust()
        .use_core()
        .generate_inline_functions(true)
        .ctypes_prefix("ctypes")
        // .enable_cxx_namespaces()
        // .raw_line("pub use self::root::*;")
        .header("../nrf5_sdk/components/softdevice/s132/headers/ble.h")
        .clang_arg("-DNRF52832_XXAA")
        .clang_arg("-DSVCALL_AS_NORMAL_FUNCTION")
        .clang_arg("-I../nrf5_sdk/components/device")
        .clang_arg("-D__STATIC_INLINE= ")
        .clang_arg("-D__START")
        .clang_arg(env::var("TARGET").unwrap())
        // .clang_arg("-x")
        // .clang_arg("c++")
        // .clang_arg("-std=c++11")
        // .parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(&outdir);
    bindings
        .write_to_file(out_path.join("ble.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", &outdir);
}
