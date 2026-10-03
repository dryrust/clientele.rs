// This is free and unencumbered software released into the public domain.

//! Supported entry point for dependency re-exports.
//!
//! Each optional crate is available when its corresponding Clientele feature is
//! enabled. Names generally match the dependency's Rust crate name; `dotenvy`
//! requires `dotenv`, `duration_str` requires `parse-duration`, `ubyte` requires
//! `parse-byteunit`, `clap_complete` requires `completions`, `clap_mangen` requires
//! `manpages`, and `tracing_core` and `tracing_subscriber` require `tracing`. `dogma` is always
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

#[cfg(feature = "manpages")]
pub use clap_mangen;

#[cfg(feature = "dirs")]
pub use dirs;

/// Dogma 0.3, with default features disabled. Clientele's `std` and `unstable`
/// features forward to Dogma's corresponding features.
///
/// To enable additional Dogma APIs, add a direct Dogma 0.3 dependency with the
/// desired features. Types from older Dogma versions used by other dependencies
/// are distinct from those in this re-export.
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

/// Subscriber construction and formatting APIs, available with `tracing`.
///
/// With `std,tracing`, reuse Clientele's formats in a thread-local subscriber
/// without Clap or process-wide initialization:
///
/// ```
/// # #[cfg(feature = "std")]
/// # {
/// use clientele::{crates::{tracing_core, tracing_subscriber}, tracing::STDERR_PLAIN_FORMAT};
///
/// let subscriber = tracing_subscriber::fmt()
///     .event_format(STDERR_PLAIN_FORMAT.clone())
///     .with_ansi(false)
///     .with_writer(std::io::sink)
///     .finish();
/// let dispatch = tracing_core::Dispatch::new(subscriber);
/// let _guard = tracing_core::dispatcher::set_default(&dispatch);
/// # }
/// ```
#[cfg(feature = "tracing")]
pub use tracing_subscriber;

#[cfg(feature = "parse-byteunit")]
pub use ubyte;

#[cfg(feature = "wild")]
pub use wild;
