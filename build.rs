extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let blockdev = pkg_config::Config::new()
        .atleast_version("2.25")
        .probe("blockdev")
        .expect("need blockdev version 2.25 headers");

    let includes: Vec<String> = blockdev
        .include_paths
        .iter()
        .map(|path| format!("{}", path.to_string_lossy()))
        .collect();

    for include in includes.iter() {
        println!("cargo:rustc-link-search={}", include)
    }

    let libs: Vec<String> = blockdev
        .libs
        .iter()
        .map(|lib| format!("{}", lib.as_str()))
        .collect();

    for lib in libs.iter() {
        println!("cargo:rustc-link-lib={}", lib)
    }
    println!("cargo:rustc-link-lib=bd_fs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(includes.iter().map(|path| format!("-I{}", path)))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
