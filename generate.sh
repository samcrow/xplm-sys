#!/bin/bash

# Generates bindings to the XPLM SDK

# Path to Rust libraries (may need to be changed)
export LD_LIBRARY_PATH=~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib

bindgen --no-layout-tests src/combined.h \
    -- -DXPLM200 -DXPLM210 -DXPLM300 -DLIN=1 -ISDK/CHeaders/XPLM -ISDK/CHeaders/Widgets \
    > src/bindgen.rs
