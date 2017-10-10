extern crate bindgen;

use std::env;
use std::env::VarError;
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
    let sdk_version = SdkVersion::get().unwrap_or_else(|e| {
        panic!("SDK version problem: {}", e);
    });
    bindgen::builder()
        // OS specification required
        .clang_arg(format!("-D{}=1", os_name))
        // Include directories
        .clang_arg("-ISDK/CHeaders/XPLM")
        .clang_arg("-ISDK/CHeaders/Widgets")
        .clang_args(sdk_version.args())
        // Add headers
        .header("src/combined.h")
        // Tests can't run because the XPLM stub library is not found
        .layout_tests(false)
        // Interpret all XPLM enum as constants
        // (like the headers)
        .constified_enum("*")
}

/// Returns true if a feature with a provided name is enabled in the current build.
fn feature_enabled(name: &str) -> Result<bool, VarError> {
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

enum SdkVersion {
    Sdk100,
    Sdk200,
    Sdk210,
    Sdk300,
}

static SDK100_ARGS: [&str; 0] = [];
static SDK200_ARGS: [&str; 1] = ["-DXPLM200"];
static SDK210_ARGS: [&str; 2] = ["-DXPLM200", "-DXPLM210"];
static SDK300_ARGS: [&str; 3] = ["-DXPLM200", "-DXPLM210", "-DXPLM300"];

impl SdkVersion {
    pub fn get() -> Result<SdkVersion, SdkVersionError> {
        let feature_xplm200 = feature_enabled("xplm200")?;
        let feature_xplm210 = feature_enabled("xplm210")?;
        let feature_xplm300 = feature_enabled("xplm300")?;
        match (feature_xplm200, feature_xplm210, feature_xplm300) {
            (false, false, false) => Ok(SdkVersion::Sdk100),
            (true, false, false) => Ok(SdkVersion::Sdk200),
            (false, true, false) => Ok(SdkVersion::Sdk210),
            (false, false, true) => Ok(SdkVersion::Sdk300),
            _ => Err(SdkVersionError::Feature("Only one of the xplm200, xplm210, or xplm300 features may be enabled"))
        }
    }

    /// Returns the compiler arguments required to enable the features of this version
    pub fn args(&self) -> &[&str] {
        match *self {
            SdkVersion::Sdk100 => &SDK100_ARGS,
            SdkVersion::Sdk200 => &SDK200_ARGS,
            SdkVersion::Sdk210 => &SDK210_ARGS,
            SdkVersion::Sdk300 => &SDK300_ARGS,
        }
    }
}

#[derive(Debug)]
enum SdkVersionError {
    Var(VarError),
    Feature(&'static str),
}

impl std::fmt::Display for SdkVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SdkVersionError::Var(ref inner) => std::fmt::Display::fmt(inner, f),
            SdkVersionError::Feature(message) => std::fmt::Display::fmt(message, f),
        }
    }
}

impl std::error::Error for SdkVersionError {
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            SdkVersionError::Var(ref inner) => Some(inner),
            SdkVersionError::Feature(_) => None,
        }
    }
    fn description(&self) -> &str {
        match *self {
            SdkVersionError::Var(ref inner) => inner.description(),
            SdkVersionError::Feature(message) => message,
        }
    }
}

impl From<VarError> for SdkVersionError {
    fn from(inner: VarError) -> Self {
        SdkVersionError::Var(inner)
    }
}
