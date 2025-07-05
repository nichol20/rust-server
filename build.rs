use std::env;
use std::process::Command;

fn main() {
    // invoke `make` in src/lib/c
    let status = Command::new("make")
        .current_dir("src/lib/c")
        .arg("static")
        .status()
        .expect("failed to run make");
    assert!(status.success());

    // tell Cargo where to find and how to link
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/src/lib/c", manifest);
    println!("cargo:rustc-link-lib=static=mathlib");
    println!("cargo:rerun-if-changed=src/lib/c/mathlib.c");
    println!("cargo:rerun-if-changed=src/lib/c/mathlib.h");
}
