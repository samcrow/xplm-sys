use std::env;
use std::path::PathBuf;

fn main() {
    link_libraries();
    use_bindgen();
}

/// On macOS and Windows targets, links the XPLM libraries
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

fn use_bindgen() {
    let bindings = bindgen::Builder::default()
        .header("src/combined.h")
        .layout_tests(false)
        .clang_args(["-DXPLM200", "-DXPLM210", "-DXPLM300", "-DXPLM301", "-DXPLM303", "-DXPLM400", "-DLIN=1", "-ISDK/CHeaders/XPLM", "-ISDK/CHeaders/Widgets"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindgen.rs")
        .expect("Couldn't write bindings!");
}
