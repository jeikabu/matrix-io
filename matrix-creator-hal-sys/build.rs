fn main() {
    link_nng();
    bindgen_generate();
}

fn link_nng() {
    // Run cmake to build
    let mut config = cmake::Config::new("matrix-creator-hal");
    let dst = config.build();

    // Check output of `cargo build --verbose`, should see something like:
    // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
    // That contains output from cmake
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     dst.join("lib").display()
    // );
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     dst.join("lib64").display()
    // );

    println!("cargo:rustc-link-lib=dylib=matrix_creator_hal");
}

#[cfg(feature = "bindgen_generate")]
fn bindgen_generate() {
    use std::{env, path::PathBuf};

    let mut builder = bindgen::Builder::default()
        .clang_arg("-Imatrix-creator-hal/cpp/driver/")
        .header("wrapper.hpp")
        .derive_default(true)
        // Generate `pub const NNG_UNIT_EVENTS` instead of `nng_unit_enum_NNG_UNIT_EVENTS`
        .prepend_enum_name(false)
        // Generate `pub enum ...` instead of multiple `pub const ...`
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .use_core()
        .whitelist_function("matrix_.*")
        .whitelist_type("matrix_.*")
        .whitelist_var("matrix_.*")
        // Layout tests are non-portable; 64-bit tests are "wrong" size on 32-bit and always fail.
        // Don't output tests if we're regenerating `src/bindings.rs` (shared by all platforms when bindgen not used)
        .layout_tests(!cfg!(feature = "source-update-bindings"));

    if !cfg!(feature = "std") {
        // no_std support
        // https://rust-embedded.github.io/book/interoperability/c-with-rust.html#automatically-generating-the-interface
        builder = builder.ctypes_prefix("cty")
    }
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("src").join("bindings.rs"))
        .expect("Couldn't write bindings");
}

#[cfg(not(feature = "bindgen_generate"))]
fn bindgen_generate() {
    // Nothing
}
