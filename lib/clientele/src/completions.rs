// This is free and unencumbered software released into the public domain.

//! Generate shell completion scripts from the same Clap command used for parsing.
//!
//! Requires `completions`, which enables `clap` and `std`. This feature is not
//! included in `all` or defaults. For a consumer's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! clientele = { version = "0.4", default-features = false, features = ["completions"] }
//! ```
//!
//! These are re-exports of `clap_complete`'s static generators for Bash, Elvish,
//! Fish, PowerShell, and Zsh. Generation writes a script; it does not install it,
//! execute a shell, or add a CLI option automatically. Supply the installed binary
//! name explicitly. [`generate`] builds and mutates the command; clone it first
//! if you need to preserve the original command state.
//!
//! ```
//! use clientele::{
//!     crates::clap::{self, CommandFactory, Parser},
//!     completions::{generate, Shell},
//!     StandardOptions,
//! };
//!
//! #[derive(Parser)]
//! struct Options {
//!     #[command(flatten)]
//!     flags: StandardOptions,
//! }
//!
//! let mut script = Vec::new();
//! generate(Shell::Bash, &mut Options::command(), "my-app", &mut script);
//! assert!(String::from_utf8(script)?.contains("--verbose"));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Output errors and panics
//!
//! The upstream [`generate`] function can panic on writer errors or an invalid
//! Clap command definition. Generate to a `Vec<u8>` and then use
//! [`std::io::Write::write_all`] if you need to handle output errors yourself.
//! [`generate_to`] creates a shell-named file in an existing output directory and
//! returns file-creation errors as `std::io::Result`; generation can still panic.
//! The complete upstream API is also available at
//! [`crate::crates::clap_complete`].

pub use clap_complete::{Generator, Shell, generate, generate_to};
