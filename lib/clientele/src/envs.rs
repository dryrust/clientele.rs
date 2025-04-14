// This is free and unencumbered software released into the public domain.

pub mod cargo;
pub use cargo::*;

pub mod conda;
pub use conda::*;

pub mod git;
pub use git::*;

pub mod macos;
pub use macos::*;

pub mod openssl;
pub use openssl::*;

pub mod posix;
pub use posix::*;

pub mod python;
pub use python::*;

pub mod ruby;
pub use ruby::*;

pub mod windows;
pub use windows::*;

pub mod xdg;
pub use xdg::*;

#[cfg(feature = "std")]
pub fn var(key: impl AsRef<std::ffi::OsStr>) -> Option<String> {
    use std::env::VarError::*;
    match std::env::var(key) {
        Err(NotPresent | NotUnicode(_)) => None,
        Ok(value) if value.trim().is_empty() => None,
        Ok(value) => Some(String::from(value.trim())), // TODO: remove the extra allocation
    }
}

#[cfg(not(feature = "std"))]
pub fn var(_key: impl AsRef<std::ffi::OsStr>) -> Option<String> {
    None // environment variables not supported
}
