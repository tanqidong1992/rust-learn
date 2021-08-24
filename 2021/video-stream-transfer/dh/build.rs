extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    //cargo:rustc-link-search=[KIND=]PATH
    let cargoManifestDir=env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=all={}", cargoManifestDir+"/x64");

    println!("cargo:rustc-link-lib=dylib={}", "avnetsdk");
    println!("cargo:rustc-link-lib=dylib={}", "dhconfigsdk");
    println!("cargo:rustc-link-lib=dylib={}", "dhnetsdk");
    println!("cargo:rustc-link-lib=dylib={}", "StreamConvertor");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .derive_debug(true)
        .clang_args(["-x","c++"])
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    //https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}