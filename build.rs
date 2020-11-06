// $env:LIBCLANG_PATH="C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\Llvm\x64\bin\libclang.dll"

extern crate bindgen;

use cmake;
use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // Builds the azure iot sdk, installing it
    // into $OUT_DIR
    use cmake::Config;

    let family: String;

    // Here because once again there is not a make install step in the Linux build
    let no_build_target: bool;

    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap().eq("windows") {
        family = "WIN32".to_string();
        no_build_target = false;
    } else {
        family = "POSIX".to_string();
        no_build_target = true;
    }

    let root = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let profile: String;

    if env::var("PROFILE").unwrap().eq("release") {
        profile = "Release".to_string();
    } else {
        profile = "Debug".to_string();
    }

    let _dst = Config::new("azure-sdk-for-c")
        .no_build_target(no_build_target)
        .build_target("ALL_BUILD")
        .define("AZ_PLATFORM_IMPL", &family)
        .build();

    let root_path: PathBuf = [
        &out_dir,
        &"build".to_string(),
        &"sdk".to_string(),
        &"src".to_string(),
        &"azure".to_string(),
    ]
    .iter()
    .collect();

    let mut core_path: PathBuf = root_path.clone();
    core_path.push("core");

    let mut iot_path: PathBuf = root_path.clone();
    iot_path.push("iot");

    let mut platform_path: PathBuf = root_path.clone();
    platform_path.push("platform");

    if family == "WIN32" {
        core_path.push(&profile);
        iot_path.push(&profile);
        platform_path.push(&profile);
    }

    println!(
        "cargo:rustc-link-search=native={}",
        core_path.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        iot_path.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        platform_path.to_str().unwrap()
    );

    // Tell cargo to tell rustc to link the azureiot libraries.
    println!("cargo:rustc-link-lib=az_iot_hub");
    println!("cargo:rustc-link-lib=az_iot_common");
    println!("cargo:rustc-link-lib=az_core");
    println!("cargo:rustc-link-lib=az_iot_provisioning");
    println!("cargo:rustc-link-lib=az_nohttp");
    println!("cargo:rustc-link-lib=az_{}", family.to_lowercase());

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut clang_args: Vec<String> = Vec::new();

    // Add clang arg that we will always need
    clang_args.push(format!("-I{}/azure-sdk-for-c/sdk/inc", root));

    // Get any additional arguments provided by the user
    let clang_input = Path::new("clangargs.txt");

    if clang_input.exists() {
        if !clang_input.is_file() {
            println!("Ignoring clangargs.txt - not a file");
        } else {
            let file = File::open(clang_input).expect("Failed to open file");
            let reader = BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        if l.starts_with("-") {
                            println!("Adding {} to clang args", l);
                            clang_args.push(l);
                        }
                    },
                    Err(e) => panic!(e),
                }
            }
        }
    } else {
        println!("No clangargs.txt found");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Add additional clang arguments - see clangargs_sample.txt
        .clang_args(clang_args)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let mut path2 = PathBuf::from(&out_dir);
    path2.push("bindings.rs");
    bindings
        .write_to_file(path2)
        .expect("Couldn't write bindings!");
}
