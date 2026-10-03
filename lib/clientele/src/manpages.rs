// This is free and unencumbered software released into the public domain.

//! Generate ROFF manual pages from a Clap command definition.
//!
//! Requires `manpages`, which enables `clap` and `std`. This feature is excluded
//! from `all` and defaults and does not enable shell completions. For a consumer:
//!
//! ```toml
//! [dependencies]
//! clientele = { version = "0.4", default-features = false, features = ["manpages"] }
//! ```
//!
//! [`Man`] is re-exported from `clap_mangen`. It takes ownership of a command
//! definition and renders its name, help, options, version, and subcommand
//! references. It neither invokes a man-page viewer nor installs files or adds
//! CLI options. The default manual section is `1`; use [`Man::section`] to
//! customize it. A page lists subcommands but does not inline their full option
//! documentation; generate separate pages for those commands when needed.
//!
//! ```
//! use clientele::{
//!     crates::clap::{self, CommandFactory, Parser},
//!     manpages::Man,
//!     StandardOptions,
//! };
//!
//! #[derive(Parser)]
//! #[command(name = "my-app", about = "Manage resources", version = "1.0")]
//! #[command(disable_version_flag = true)] // StandardOptions supplies --version.
//! struct Options {
//!     #[command(flatten)]
//!     flags: StandardOptions,
//! }
//!
//! let page = Man::new(Options::command());
//! let mut roff = Vec::new();
//! page.render(&mut roff)?;
//! assert!(String::from_utf8(roff)?.contains("Manage resources"));
//! assert_eq!(page.get_filename(), "my-app.1");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Use [`Man::generate_to`] to write a page into an existing output directory.
//! File creation and rendering return `std::io::Result`; [`Man::render`] propagates
//! writer errors and may have written partial output before failing. Invalid Clap
//! definitions can panic during command construction. Full upstream APIs,
//! including recursive generation, are available at [`crate::crates::clap_mangen`].

pub use clap_mangen::Man;
