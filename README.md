# xplm-sys: Rust bindings for the X-Plane plugin SDK #

This library provides Rust bindings to the X-Plane plugin SDK.

## SDK versions ##

The X-Plane plugin SDK has four versions:

* Version 1.0, supported by X-Plane 6.70 and up
* Version 2.0, supported by X-Plane 9.00 and up
* Version 2.1, supported by X-Plane 10.00 and up
* Version 3.0, supported by X-Plane 11.10 and up

Each version adds a few new features. To enable an SDK version, use the `xplm200`,
 `xplm210`, or `xplm300` cargo feature. Only one feature should be enabled.

## Documentation ##

The types and functions are documented on [the X-Plane plugin API website](http://developer.x-plane.com/sdk/).

## Compiling and linking ##

This library currently can be compiled on Mac OS, Linux, and Windows.

On Mac OS and Windows, plugins must be dynamically linked with libraries that
provide stub implementations of the SDK functions. This crate includes those
libraries and tells Cargo to link them.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Files in the SDK folder are provided under a separate license, provided in
[SDK/license.txt](SDK/license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
