extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    //cargo:rustc-link-search=[KIND=]PATH
    let cargoManifestDir=env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=all={}", cargoManifestDir+"/x64");
    //println!("cargo:rustc-flags=-l dylib={}", "/home/tqd/workspaces/rust-learn/2021/video-stream-transfer/dh/x64");
    //println!("cargo:rustc-flags=-L {}", "/home/tqd/workspaces/rust-learn/2021/video-stream-transfer/dh/x64");、
    //println!("cargo:rustc-flags=-l dylib={}", "/home/tqd/workspaces/rust-learn/2021/video-stream-transfer/dh/x64/libavnetsdk.so");
    //println!("cargo:rustc-flags=-l dylib={}", "/home/tqd/workspaces/rust-learn/2021/video-stream-transfer/dh/x64/libdhconfigsdk.so");
    //println!("cargo:rustc-flags=-l dylib={}", "/home/tqd/workspaces/rust-learn/2021/video-stream-transfer/dh/x64/libdhnetsdk.so");
    //println!("cargo:rustc-flags=-l dylib={}", "/home/tqd/workspaces/rust-learn/2021/video-stream-transfer/dh/x64/libStreamConvertor.so");
    //println!("cargo:rustc-link-lib={}", "libavnetsdk");
    //println!("cargo:rustc-link-lib={}", "libdhconfigsdk");
    //println!("cargo:rustc-link-lib={}", "libdhnetsdk");
    //println!("cargo:rustc-link-lib={}", "libStreamConvertor");

    println!("cargo:rustc-link-lib=dylib={}", "avnetsdk");
    println!("cargo:rustc-link-lib=dylib={}", "dhconfigsdk");
    println!("cargo:rustc-link-lib=dylib={}", "dhnetsdk");
    println!("cargo:rustc-link-lib=dylib={}", "StreamConvertor");

    //println!("-include {}","linux/types.h");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .derive_debug(true)
        //.clang_arg("-x c++")
        .clang_args(["-x","c++"
           // "--dynamic-link-require-all"
            //"--dynamic-loading","libavnetsdk",
           // "--dynamic-loading","libdhconfigsdk",
            //"--dynamic-loading","libdhnetsdk",
            //"--dynamic-loading","libStreamConvertor",
        ])
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
    /**
    let out_path = String::from("/home/tqd/workspaces/rust-learn/2021/video-stream-transfer/target");
    bindings
        .write_to_file(out_path+"bindings.rs")
        .expect("Couldn't write bindings!");
    */
    //https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}