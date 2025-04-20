// This is free and unencumbered software released into the public domain.

//! This crate provides CLI support utilities.
//!
//! ```edition2021
//! # use clientele::*;
//! ```

//#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

#[doc(hidden)]
mod prelude;

#[cfg(feature = "std")]
mod args;
#[cfg(feature = "std")]
pub use args::*;

#[cfg(feature = "camino")]
pub use camino::{Utf8Path, Utf8PathBuf};

#[cfg(feature = "dotenv")]
pub use dotenvy::dotenv;

#[cfg(all(feature = "std", feature = "getenv"))]
pub use getenv as envs;

mod features;
pub use features::*;

#[cfg(all(feature = "std", feature = "clap"))]
mod options;
#[cfg(all(feature = "std", feature = "clap"))]
pub use options::*;

#[cfg(all(feature = "std", feature = "getenv", feature = "camino"))]
pub mod paths;

#[cfg(all(feature = "std", feature = "subcommands"))]
mod subcommands;
#[cfg(all(feature = "std", feature = "subcommands"))]
pub use subcommands::*;

mod sysexits;
pub use sysexits::*;

#[doc(hidden)]
pub mod crates {
    #[cfg(feature = "argfile")]
    pub use argfile;
    #[cfg(feature = "camino")]
    pub use camino;
    #[cfg(feature = "clap")]
    pub use clap;
    #[cfg(feature = "dirs")]
    pub use dirs;
    pub use dogma;
    #[cfg(feature = "dotenv")]
    pub use dotenvy;
    #[cfg(feature = "parse-duration")]
    pub use duration_str;
    #[cfg(feature = "error-stack")]
    pub use error_stack;
    #[cfg(feature = "getenv")]
    pub use getenv;
    #[cfg(feature = "serde")]
    pub use serde;
    #[cfg(feature = "serde-json")]
    pub use serde_json;
    #[cfg(feature = "tokio")]
    pub use tokio;
    #[cfg(feature = "tracing")]
    pub use tracing;
    #[cfg(feature = "parse-byteunit")]
    pub use ubyte;
    #[cfg(feature = "wild")]
    pub use wild;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
