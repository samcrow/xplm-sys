extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let builder = configure_bindings();
    builder.generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("bindgen.rs"))
        .unwrap();
}

fn configure_bindings() -> bindgen::Builder {
    let os_name = get_os_str();
    bindgen::builder()
        // OS specification required
        .clang_arg(format!("-D{}=1", os_name))
        // Include directories
        .clang_arg("-ISDK/CHeaders/XPLM")
        .clang_arg("-ISDK/CHeaders/Widgets")
        .no_unstable_rust()
        // Add headers
        .header("src/combined.h")
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
