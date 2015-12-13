use std::env::var;
///
/// Tells Cargo to link the correct libraries
///

fn main() {
    let target = var("TARGET").ok().expect("No TARGET environment variable");

    if target.contains("-apple-") {
        println!("cargo:rustc-link-search=framework=SDK/Libraries/Mac/");
        println!("cargo:rustc-link-lib=framework=XPLM");
        println!("cargo:rustc-link-lib=framework=XPWidgets");
    }
    else if target.contains("-linux-") {
        // Do nothing for Linux
    }
    else {
        panic!("Unknown operating system (target = {}) is not supported", target);
    }
}
