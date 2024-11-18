#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(clippy::all)]

#[cfg(any(target_os = "ios", target_os = "android"))]
include!(concat!(env!("OUT_DIR"), "/mobile_bindings.rs"));
