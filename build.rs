extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    generate_bindings();
    link_libraries();
}

fn generate_bindings() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let builder = configure_bindings();
    builder.generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("bindgen.rs"))
        .unwrap();
}

fn configure_bindings() -> bindgen::Builder {
    let os_name = get_os_str();
    let mut builder = bindgen::builder()
        // OS specification required
        .clang_arg(format!("-D{}=1", os_name))
        // Include directories
        .clang_arg("-ISDK/CHeaders/XPLM")
        .clang_arg("-ISDK/CHeaders/Widgets")
        .no_unstable_rust()
        // Add headers
        .header("src/combined.h");

    // SDK 2.0 and 2.1
    if feature_enabled("xplm200").unwrap() {
        builder = builder.clang_arg("-DXPLM200");
    }
    if feature_enabled("xplm210").unwrap() {
        builder = builder.clang_arg("-DXPLM210");
    }
    builder
}

/// Returns true if a feature with a provided name is enabled in the current build.
fn feature_enabled(name: &str) -> Result<bool, ::std::env::VarError> {
    let transformed_name = name.to_uppercase().replace('-', "_");
    let var_name = format!("CARGO_FEATURE_{}", transformed_name);
    match env::var(&var_name) {
        Ok(_) => Ok(true),
        Err(::std::env::VarError::NotPresent) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Returns "MAC", "LIN", or "IBM", depending on the target operating system
fn get_os_str() -> &'static str {
    let target = env::var("TARGET").unwrap();
    if target.contains("-apple-") {
        "APL"
    }
    else if target.contains("-linux-") {
        "LIN"
    }
    else if target.contains("-windows-") {
        "IBM"
    } else {
        panic!("Target operating system not Mac OS, Linux, or Windows")
    }
}

/// On Mac OS and Windows targets, links the XPLM libraries
fn link_libraries() {
    // Get the absolute path to this crate, so that linking will work when done in another folder
    let crate_path = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let crate_path = Path::new(&crate_path);
    let target = env::var("TARGET").unwrap();

    if target.contains("-apple-") {
        let library_path = crate_path.join("SDK/Libraries/Mac");
        println!("cargo:rustc-link-search=framework={}", library_path.to_str().unwrap());
        println!("cargo:rustc-link-lib=framework=XPLM");
        println!("cargo:rustc-link-lib=framework=XPWidgets");
    } else if target.contains("-linux-") {
        // Do nothing for Linux
    } else if target.contains("-windows-") {
        let library_path = crate_path.join("SDK/Libraries/Win");
        println!("cargo:rustc-link-search={}", library_path.to_str().unwrap());
        if target.contains("x86_64") {
            println!("cargo:rustc-link-lib=XPLM_64");
            println!("cargo:rustc-link-lib=XPWidgets_64");
        } else {
            println!("cargo:rustc-link-lib=XPLM");
            println!("cargo:rustc-link-lib=XPWidgets");
        }
    }
    else {
        panic!("Target operating system not Mac OS, Linux, or Windows")
    }
}
