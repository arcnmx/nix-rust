//! Rust friendly bindings to the various *nix system functions.
//!
//! Modules are structured according to the C header file that they would be
//! defined in.
#![crate_name = "nix"]
#![cfg(unix)]
#![allow(non_camel_case_types)]
// latest bitflags triggers a rustc bug with cross-crate macro expansions causing dead_code
// warnings even though the macro expands into something with allow(dead_code)
#![allow(dead_code)]
#![deny(warnings)]

#[macro_use]
extern crate bitflags;

extern crate libc;

#[cfg(test)]
extern crate nix_test as nixtest;

// Re-exports
pub use libc::{c_int, c_void};
pub use errno::{Errno, Result};
pub use nix_string::NixString;

#[macro_use]
mod nix_string;

pub mod errno;
pub mod features;
pub mod fcntl;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod mount;

#[cfg(any(target_os = "linux"))]
pub mod mqueue;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod poll;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod sched;

pub mod sys;
pub mod unistd;
