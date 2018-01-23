extern crate cmake;

use std::{fs, str};

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            name
        );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn build_cjieba() {
    let dst = cmake::Config::new("cppjieba-cabi")
        .build_target("cjieba_static")
        .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=cjieba_static");
}

fn link_cpp() {
    // XXX: static link libc++?
    if cfg!(any(target_os = "macos", target_os = "freebsd")) {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=gcc");
    }
}

fn main() {
    fail_on_empty_directory("cppjieba-cabi");
    fail_on_empty_directory("cppjieba-cabi/cppjieba");
    build_cjieba();
    link_cpp();
}
