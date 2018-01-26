use std::env;
use std::path::Path;

fn main() {
    link_libraries();
}

//fn configure_bindings() -> bindgen::Builder {
//    let os_name = get_os_str();
//    let sdk_version = SdkVersion::get().unwrap_or_else(|e| {
//        panic!("SDK version problem: {}", e);
//    });
//    bindgen::builder()
//        // OS specification required
//        .clang_arg(format!("-D{}=1", os_name))
//        // Include directories
//        .clang_arg("-ISDK/CHeaders/XPLM")
//        .clang_arg("-ISDK/CHeaders/Widgets")
//        .clang_args(sdk_version.args())
//        // Add headers
//        .header("src/combined.h")
//        // Tests can't run because the XPLM stub library is not found
//        .layout_tests(false)
//        // Interpret all XPLM enum as constants
//        // (like the headers)
//        .constified_enum("*")
//}

/// On Mac OS and Windows targets, links the XPLM libraries
fn link_libraries() {
    // Get the absolute path to this crate, so that linking will work when done in another folder
    let crate_path = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let crate_path = Path::new(&crate_path);
    let target = env::var("TARGET").unwrap();

    if target.contains("-apple-") {
        let library_path = crate_path.join("SDK/Libraries/Mac");
        println!(
            "cargo:rustc-link-search=framework={}",
            library_path.to_str().unwrap()
        );
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
    } else {
        panic!("Target operating system not Mac OS, Linux, or Windows")
    }
}
