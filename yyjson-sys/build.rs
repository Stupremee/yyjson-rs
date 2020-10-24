use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build = dst.join("build");

    let mut cc = cc::Build::new();
    cc.warnings(false)
        .out_dir(&build)
        .include("src/yyjson/src")
        .file("src/yyjson/src/yyjson.c");

    cc.compile("yyjson");

    let bindings = bindgen::Builder::default()
        .header("src/yyjson/src/yyjson.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("yyjson_.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
