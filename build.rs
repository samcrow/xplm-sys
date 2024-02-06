use std::env;
use std::path::PathBuf;

extern crate bindgen;

fn main() {
    let crate_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let library_path = crate_path.join("SDK\\Libraries\\Win");
    println!("cargo:rustc-link-search={}", library_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=XPLM_64");
    println!("cargo:rustc-link-lib=XPWidgets_64");

    let bindings = bindgen::Builder::default()
        .header("src/combined.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindgen.rs")
        .expect("Couldn't write bindings!");
}
