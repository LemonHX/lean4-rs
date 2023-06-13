use std::{path::PathBuf, process::Command};

fn main() {
    let lean_output = Command::new("lean")
        .args(["--print-prefix"])
        .output()
        .expect("Failed to execute lean");
    if !lean_output.status.success() {
        panic!(
            "Command \"lean --print-prefix\" exited unsuccessfully: error {}",
            lean_output.status
        );
    }

    let mut lean_dir = PathBuf::from(String::from_utf8(lean_output.stdout)
        .expect("Path returned by \"lean --print-prefix\" is invalid UTF-8; this is currently not supported")
        .trim());
    if cfg!(target_os = "windows") {
        lean_dir.push("bin")
    } else {
        // Default to Unix-like, of course
        lean_dir.push("lib/lean")
    }

    // Step 2: check libleanshared.so/libleanshared.dylib/libleanshared.dll is actually there, just for cleaner error messages
    let mut shared_lib = lean_dir.clone();
    let exists = if cfg!(target_os = "windows") {
        shared_lib.push("libleanshared.dll");
        shared_lib.exists()
    } else if cfg!(target_os = "mac") {
        shared_lib.push("libleanshared.dylib");
        shared_lib.exists()
    } else if cfg!(unix) {
        shared_lib.push("libleanshared.so");
        shared_lib.exists()
    } else {
        true
    };
    if !exists {
        panic!(
            "{} was not found. We errored, as this would probably cause a linking failure later",
            shared_lib.display()
        );
    }

    // Step 3: actually link with the library
    println!("cargo:rustc-link-search={}", lean_dir.display());
    println!("cargo:rustc-link-lib=leanshared");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lean_dir.display());



    let bindings = bindgen::Builder::default()
        .header("lean.h")
        .wrap_static_fns(true)
        .wrap_static_fns_path("wrap_static_fns.c")
        .wrap_static_fns_suffix("_rs_extern")
        .generate()
        .expect("Unable to generate bindings");
    
    // use cc to compile the C wrapper
    cc::Build::new()
        .file("wrap_static_fns.c")
        .compile("wrap_static_fns");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
