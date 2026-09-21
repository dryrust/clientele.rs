// This is free and unencumbered software released into the public domain.

//! Logging formats and subscriber initialization.
//!
//! Available with the `std` and `tracing` features. The formats can be used
//! without Clap; `init_tracing_subscriber` additionally requires `clap`.

#[cfg(feature = "clap")]
use crate::StandardOptions;
use std::sync::LazyLock;
use tracing_subscriber::fmt::{
    format::{Compact, Format},
    time::SystemTime,
};

/// Compact, untimed formatting without event levels or source metadata.
pub const STDERR_PLAIN_FORMAT: LazyLock<Format<Compact, ()>> = LazyLock::new(|| {
    tracing_subscriber::fmt::format()
        .compact()
        .without_time()
        .with_target(false)
        .with_level(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
});

/// Compact, untimed formatting retaining event levels and targets.
pub const STDERR_DEBUG_FORMAT: LazyLock<Format<Compact, ()>> =
    LazyLock::new(|| tracing_subscriber::fmt::format().compact().without_time());

/// Initializes `tracing_subscriber` based on the given options.
///
/// Requires the `clap` feature in addition to `std` and `tracing`.
///
/// # Panics
///
/// Panics if a global tracing subscriber has already been installed.
///
/// # Examples
///
/// ```no_run
/// use clientele::{crates::clap::Parser, tracing::init_tracing_subscriber, StandardOptions};
///
/// #[derive(Parser)]
/// struct Options {
///     #[command(flatten)]
///     flags: StandardOptions,
/// }
///
/// let options = Options::parse();
/// init_tracing_subscriber(&options.flags);
/// ```
#[cfg(feature = "clap")]
pub fn init_tracing_subscriber(options: &StandardOptions) {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(options)
        .event_format(if options.debug {
            STDERR_DEBUG_FORMAT.clone()
        } else {
            STDERR_PLAIN_FORMAT.clone()
        })
        .init();
}
