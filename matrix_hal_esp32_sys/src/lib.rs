
// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Disable clippy since this is all bindgen generated code
#![allow(clippy::all)]

#![no_std]

include!("bindings.rs");
pub use root::*;