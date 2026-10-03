// This is free and unencumbered software released into the public domain.

//! This crate provides CLI support utilities.
//!
//! # Feature dependencies
//!
//! This crate requires Rust's standard library even with default features disabled.
//! The `std` feature gates APIs and dependency integrations; disabling it does not
//! provide `no_std` support.
//!
//! The `clap` feature also enables `std`. With `error-stack`, [`SysexitsError`]
//! can be used as a report context with or without the `std` feature, including
//! when default features are disabled.
//!
//! Tracing formats require `std` and `tracing`; the options-based tracing
//! initializers additionally require `clap`.
//! The `color` feature enables ANSI support in Clap and tracing when those
//! optional dependencies are enabled, without enabling either dependency itself.
//!
//! The `parse-datetime` feature is reserved: it currently enables no dependencies
//! and provides no date/time parsing API or re-export. It is included by `parse`
//! (and therefore by `all` and the default features), but enabling it alone has
//! no effect. Applications needing date/time parsing must supply their own parser.
//!
//! Shell completion generation is available through `clientele::completions` with the
//! opt-in `completions` feature, which enables `clap` and `std`. It is excluded
//! from `all` and the default features.
//! Man-page generation is available through `clientele::manpages` with the
//! independent opt-in `manpages` feature, also enabling `clap` and `std` and
//! excluded from `all` and defaults.
//!
//! ```edition2021
//! # use clientele::*;
//! ```

//#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

// Cargo adjusts the README path when packaging the crate.
#[cfg(doctest)]
#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/",
    env!("CARGO_PKG_README")
))]
pub struct ReadmeDoctests;

pub use known_errors::sysexits::{SysexitsError, SysexitsResult};

#[cfg(feature = "std")]
pub use known_errors::{abort, sysexits::exit};

#[doc(hidden)]
mod prelude;

#[cfg(feature = "std")]
mod args;
#[cfg(feature = "std")]
pub use args::*;

#[cfg(feature = "camino")]
pub use camino::{Utf8Path, Utf8PathBuf};

#[cfg(feature = "clap")]
mod clap;
#[cfg(feature = "clap")]
pub use clap::*;

#[cfg(feature = "dotenv")]
pub use dotenvy::dotenv;

#[cfg(all(feature = "std", feature = "getenv"))]
pub use getenv as envs;

#[cfg(all(feature = "std", feature = "clap"))]
pub mod options;
#[cfg(all(feature = "std", feature = "clap"))]
pub use options::*;

#[cfg(feature = "std")]
pub mod paths;

#[cfg(all(feature = "std", feature = "subcommands"))]
mod subcommands;
#[cfg(all(feature = "std", feature = "subcommands"))]
#[doc(inline)]
pub use subcommands::*;

pub mod crates;

#[cfg(feature = "completions")]
pub mod completions;

#[cfg(feature = "manpages")]
pub mod manpages;

mod color;
pub use color::*;

#[cfg(all(feature = "std", feature = "tracing"))]
pub mod tracing;
