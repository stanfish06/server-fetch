use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rerun-if-changed=bindings.c");
    println!("cargo:rerun-if-changed=libbindings.a");
    
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=wtmpdb");
    
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", manifest_dir);
    println!("cargo:rustc-link-lib=static=bindings");
    
    println!("cargo:rustc-link-arg-bins=-L");
    println!("cargo:rustc-link-arg-bins={}", manifest_dir);
    println!("cargo:rustc-link-arg-bins=-lbindings");
    
    let bindings = bindgen::Builder::default()
        .header("bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
