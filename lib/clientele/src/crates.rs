// This is free and unencumbered software released into the public domain.

//! Supported entry point for dependency re-exports.
//!
//! Each optional crate is available when its corresponding Clientele feature is
//! enabled. Names generally match the dependency's Rust crate name; `dotenvy`
//! requires `dotenv`, `duration_str` requires `parse-duration`, `ubyte` requires
//! `parse-byteunit`, `clap_complete` requires `completions`, and `tracing_core`
//! requires `tracing`. `dogma` is always
//! available. Enabling a re-export does not necessarily enable Clientele's
//! higher-level helpers; consult their feature requirements separately.
//!
//! For example, with `clap`, import `clientele::crates::clap::Parser` to use the
//! same Clap version as Clientele. Only the re-exports listed here are provided;
//! not every transitive dependency is exposed.

#[cfg(feature = "argfile")]
pub use argfile;

#[cfg(feature = "camino")]
pub use camino;

#[cfg(feature = "clap")]
pub use clap;

#[cfg(feature = "completions")]
pub use clap_complete;

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

#[cfg(feature = "gofer")]
pub use gofer;

#[cfg(feature = "serde")]
pub use serde;

#[cfg(feature = "serde-json")]
pub use serde_json;

#[cfg(feature = "tokio")]
pub use tokio;

#[cfg(feature = "tracing")]
pub use tracing_core;

#[cfg(feature = "parse-byteunit")]
pub use ubyte;

#[cfg(feature = "wild")]
pub use wild;
