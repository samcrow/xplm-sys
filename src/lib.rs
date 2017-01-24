// Copyright (c) 2015 xplm-sys developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

// Allow C-like conventions
#![allow(non_upper_case_globals,non_camel_case_types,non_snake_case)]

// Link libraries for Mac OS and Windows
#[cfg_attr(target_os = "macos", link(kind = "framework", name = "XPLM"))]
#[cfg_attr(target_os = "macos", link(kind = "framework", name = "XPWidgets"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = 32), link(name = "XPLM"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = 32), link(name = "XPWidgets"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = 64), link(name = "XPLM_64"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = 64), link(name = "XPWidgets_64"))]
extern {

}

include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
