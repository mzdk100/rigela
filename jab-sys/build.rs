extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("cargo:rerun-if-changed=allow_funcs.txt");

    cc::Build::new()
        .file("./c_jab/AccessBridgeCalls.c")
        .include("./c_jab")
        .compile("jab");

    let builder = bindgen::Builder::default()
        .header("wrapper.h");
    let builder = filter_funcs(&builder);

    let bindings = builder
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&["-D__AVX512VLFP16INTRIN_H", "-D__AVX512FP16INTRIN_H"])
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn filter_funcs(builder: &bindgen::Builder) -> bindgen::Builder {
    let mut result: bindgen::Builder = builder.clone();

    let file = File::open("allow_funcs.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        result = result.clone().allowlist_function(line);
    }

    result
}