// Copyright (c) 2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=nghttp2");
    
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .generate()
        .expect("Failed to generate bindings.");
        
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write the bindings file.");
}