// This is free and unencumbered software released into the public domain.

//! Logging formats and subscriber initialization.
//!
//! Available with the `std` and `tracing` features. The formats can be used
//! without Clap; both subscriber initializers additionally require `clap`.

#[cfg(feature = "clap")]
use crate::StandardOptions;
use std::sync::LazyLock;
use tracing_subscriber::fmt::{
    format::{Compact, Format},
    time::SystemTime,
};

/// Compact, untimed formatting without event levels or source metadata.
///
/// Available with `std,tracing`, without requiring `clap`. Initialized once and
/// shared across callers. Call `.clone()` to obtain an owned format for a
/// subscriber or to customize it without changing the shared format.
///
/// This is a static rather than a constant: the `LazyLock` itself cannot be moved
/// out or consumed. Clone the inner format instead.
///
/// ```
/// use clientele::tracing::STDERR_PLAIN_FORMAT;
///
/// let format = STDERR_PLAIN_FORMAT.clone().with_level(true);
/// ```
pub static STDERR_PLAIN_FORMAT: LazyLock<Format<Compact, ()>> = LazyLock::new(|| {
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
///
/// Available with `std,tracing`, without requiring `clap`. Initialized once and
/// shared across callers. As with [`STDERR_PLAIN_FORMAT`], call `.clone()` to
/// obtain an owned, customizable format; the static `LazyLock` cannot be moved
/// out or consumed.
pub static STDERR_DEBUG_FORMAT: LazyLock<Format<Compact, ()>> =
    LazyLock::new(|| tracing_subscriber::fmt::format().compact().without_time());

/// Initializes `tracing_subscriber` based on the given options.
///
/// Requires the `clap` feature in addition to `std` and `tracing`.
///
/// Writes to stderr using [`STDERR_PLAIN_FORMAT`], or [`STDERR_DEBUG_FORMAT`]
/// when `options.debug` is enabled. Level filtering follows [`StandardOptions`].
///
/// With the `color` feature, `options.color` is resolved for stderr using
/// [`crate::ColorChoiceExt::to_bool_for`]. Automatic color requires stderr to be a
/// terminal and `NO_COLOR` to be unset or empty; explicit `Always` and `Never`
/// choices override detection. Without `color`, ANSI output is disabled even if
/// another dependency enables ANSI support in `tracing-subscriber`.
///
/// # Panics
///
/// Panics if a global tracing subscriber has already been installed or another
/// initialization error occurs. Use [`try_init_tracing_subscriber`] to handle
/// initialization errors instead.
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
    try_init_tracing_subscriber(options).expect("Unable to install global subscriber");
}

/// Attempts to initialize the global tracing subscriber based on the given options.
///
/// Requires the `clap` feature in addition to `std` and `tracing`. Uses the same
/// stderr output, plain/debug formats, level filtering, and color handling as
/// [`init_tracing_subscriber`]. Returns `Ok(())` when initialization succeeds.
///
/// # Errors
///
/// Returns the underlying `tracing-subscriber` initialization error, including
/// when a global subscriber has already been installed. An existing subscriber
/// is not replaced. If `tracing-subscriber`'s `tracing-log` feature is enabled
/// through dependency feature unification, errors installing its log bridge are
/// also returned.
///
/// # Examples
///
/// ```no_run
/// use clientele::{crates::clap::Parser, tracing::try_init_tracing_subscriber, StandardOptions};
///
/// #[derive(Parser)]
/// struct Options {
///     #[command(flatten)]
///     flags: StandardOptions,
/// }
///
/// let options = Options::parse();
/// try_init_tracing_subscriber(&options.flags)?;
/// # Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
/// ```
#[cfg(feature = "clap")]
pub fn try_init_tracing_subscriber(
    options: &StandardOptions,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    #[cfg(feature = "color")]
    let ansi = {
        use crate::{ColorChoiceExt, ColorStream};
        options.color.to_bool_for(ColorStream::Stderr)
    };
    #[cfg(not(feature = "color"))]
    let ansi = false;

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(options)
        .event_format(if options.debug {
            STDERR_DEBUG_FORMAT.clone()
        } else {
            STDERR_PLAIN_FORMAT.clone()
        })
        .with_ansi(ansi)
        .try_init()
}
