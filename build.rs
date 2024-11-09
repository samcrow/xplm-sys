use std::{env, path::PathBuf, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=NULL"); // forces the build script to run even when no file changed and a previous build is existent.
    link_libraries();
    use_bindgen();
}

/// This function is responsible for linking the required libraries
///
/// The function determines the target operating system and adds the necessary
/// search paths and library names to the build configuration using the `cargo` command.
fn link_libraries() {
    let crate_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
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

/// Generates Rust bindings for a C header file using bindgen.
///
/// # Panics
///
/// This function will panic if it fails to generate the bindings.
fn use_bindgen() {
    let path_combined = "src/combined.h";
    let out_dir = env::var("OUT_DIR").unwrap();
    let path_bindgen = Path::new(&out_dir).join("bindgen.rs");

    let bindings = bindgen::Builder::default()
        .header(path_combined)
        .layout_tests(false)
        .clang_arg("-ISDK/CHeaders/Widgets")
        .clang_arg("-ISDK/CHeaders/XPLM")
        .clang_arg("-DLIN=1")
        .clang_arg("-DXPLM200")
        .clang_arg("-DXPLM210")
        .clang_arg("-DXPLM300")
        .clang_arg("-DXPLM301")
        .clang_arg("-DXPLM303")
        .clang_arg("-DXPLM400")
        .clang_arg("-DXPLM410")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(path_bindgen)
        .expect("Couldn't write bindings!");
}
