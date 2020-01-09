use std::{env, path::PathBuf};

fn main() {
    link_nng();
    bindgen_generate();
}

fn link_nng() {
    // Check output of `cargo build --verbose`, should see something like:
    // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
    // That contains output from cmake
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}", root.join("build/hal/").display()
    );
    println!("cargo:rustc-link-lib=static=hal");
}

#[cfg(feature = "bindgen_generate")]
fn env_var_expect(name: &str) -> String {
    env::var(name).expect(&format!("Environment variable must be set: {}", name))
}

#[cfg(feature = "bindgen_generate")]
fn bindgen_generate() {
    let _idf_path = env_var_expect("IDF_PATH");
    let esp_path = env_var_expect("ESP_PATH");

    let mut builder = bindgen::Builder::default()
        .clang_args(&[
            format!("--sysroot={}/xtensa-esp32-elf/sysroot", esp_path),
            "-I./build/include".to_owned(),
            format!("-I{}/xtensa-esp32-elf/include/c++/5.2.0/", esp_path),
            format!("-I{}/xtensa-esp32-elf/include/c++/5.2.0/xtensa-esp32-elf/", esp_path),
            "-Imatrixio_hal_esp32/components/hal/".to_owned(),
            "-D__bindgen".to_owned(),
            "-target xtensa".to_owned(),
        ])
        .derive_default(true)
        // Generate `pub const ENUM` instead of `type_ENUM`
        .prepend_enum_name(false)
        // Generate `pub enum ...` instead of multiple `pub const ...`
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .use_core()
        .ctypes_prefix("cty")
        .layout_tests(false)
        .enable_cxx_namespaces();

    builder = builder
        .header("wrapper.hpp")
        //.whitelist_function("matrix_.*")
        .whitelist_type("matrix_.*")
        //.whitelist_var("matrix_.*")
        ;

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
