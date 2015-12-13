use std::env::var;
///
/// Tells Cargo to link the correct libraries
///

fn main() {
    let target = var("TARGET").ok().expect("No TARGET environment variable");

    if target.contains("-apple-") {
        println!("cargo:rustc-link-search=framework=../XPLM/SDK/Libraries/Mac/");
        println!("cargo:rustc-link-lib=framework=XPLM");
        println!("cargo:rustc-link-lib=framework=XPWidgets");
    }
    else {
        panic!("Windows (target = {}) is not yet supported", target);
    }
}
