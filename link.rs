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
    else if target.contains("-windows-") {
        println!("cargo:rustc-link-search=SDK/Libraries/Win/");
		if target.contains("x86_64") {
			println!("cargo:rustc-link-lib=XPLM_64.lib");
			println!("cargo:rustc-link-lib=XPWidgets_64.lib");
		}
		else {
			println!("cargo:rustc-link-lib=XPLM.lib");
			println!("cargo:rustc-link-lib=XPWidgets.lib");
		}
    }
    else {
        panic!("Unknown operating system (target = {}) is not supported", target);
    }
}
