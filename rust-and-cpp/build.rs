use std::env;
use std::path::PathBuf;

fn main() {
    // Get rust-lib root path
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Include cpp-lib target dir
    println!("cargo:rustc-link-search=native={}", manifest_dir.join("../cpp-lib/target/lib").as_path().display());
    // Link static cpp-lib library
    println!("cargo:rustc-link-lib=static=cpp-lib");
    
    // This assumes all your C++ bindings are in main.rs
    println!("cargo:rerun-if-changed=src/main.rs");
    // Define path to resolve #include relative position
    let include_path = manifest_dir.join("../cpp-lib/src");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&include_path]).build().unwrap();
    b.flag_if_supported("-std=c++17")
     .compile("auto-cpp-lib");
}
