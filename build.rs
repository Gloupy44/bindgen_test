use std::env;
use std::path::PathBuf;

//  Build C static library
fn build_my_c_lib() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=my_c_lib/src/my_c_lib.h,my_c_lib/src/my_c_lib.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .pic(true)
        .static_flag(true)
        .include("./my_c_lib/include")
        .file("my_c_lib/src/my_c_lib.c")
        .compile("my_c_lib")
}

fn main() {
    // First, rebuild C static library
    build_my_c_lib();

    // Tell cargo to look for libraries in the specified directory
    println!("cargo:rustc-link-search=./my_c_lib/lib");

    // Tell cargo to tell rustc to link
    // the "my_c_lib" static library.
    println!("cargo:rustc-link-lib=my_c_lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./my_c_lib/include/my_c_lib.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("my_c_lib_bindings.rs"))
        .expect("Couldn't write bindings!");
}
