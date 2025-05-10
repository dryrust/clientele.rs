// This is free and unencumbered software released into the public domain.

//! This crate provides CLI support utilities.
//!
//! ```edition2021
//! # use clientele::*;
//! ```

//#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
pub struct ReadmeDoctests;

pub use known_errors::{
    abort,
    sysexits::{exit, SysexitsError, SysexitsResult},
};

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

#[cfg(all(feature = "std", feature = "clap"))]
mod options;
#[cfg(all(feature = "std", feature = "clap"))]
pub use options::*;

#[cfg(all(feature = "std", feature = "getenv", feature = "camino"))]
pub mod paths;

#[cfg(all(feature = "std", feature = "subcommands"))]
#[doc(hidden)]
mod subcommands;
#[cfg(all(feature = "std", feature = "subcommands"))]
#[doc(hidden)]
pub use subcommands::*;

#[doc(hidden)]
pub mod crates;
