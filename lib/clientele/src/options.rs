// This is free and unencumbered software released into the public domain.

//! Reusable CLI flags and sort-key types.
//!
//! Requires `std,clap` (`clap` enables `std`). Flatten [`StandardOptions`] into a
//! Clap parser to add common flags. The `color` feature adds the color flag;
//! `tracing` adds owned and borrowed conversions to a tracing `LevelFilter`.

pub mod sort;

extern crate std;

use clap::{ArgAction, Args};

/// Common flags for a Clap parser, also re-exported at the crate root.
///
/// Requires `std,clap`. Use `#[command(flatten)]` on a field of this type in a
/// derived parser. Defaults below are supplied by Clap when flags are absent;
/// this type does not implement `Default`.
///
/// # Flags and scope
///
/// | Field | Flags | Parsed default | Scope |
/// | --- | --- | --- | --- |
/// | `color` (requires `color`) | `--color <auto\|always\|never>` | `Auto` | Global |
/// | [`debug`](Self::debug) | `-d`, `--debug` | `false` | Global |
/// | [`license`](Self::license) | `--license` | `false` | Command-local |
/// | [`verbose`](Self::verbose) | `-v`, `--verbose` | `0` | Global |
/// | [`version`](Self::version) | `-V`, `--version` | `false` | Command-local |
///
/// Global flags are accepted on the command containing the flattened options
/// and on its subcommands, and their values are propagated by Clap. Command-local
/// flags belong only to the command where these options are flattened; they are
/// not inherited by subcommands.
///
/// Parsing only records the flags. The application is responsible for printing
/// license/version information and choosing whether to exit, applying color
/// preferences, and initializing logging. The `version` field is a boolean flag,
/// not Clap's automatic version action. If the containing command enables that
/// action through version metadata, disable it with `disable_version_flag = true`
/// to avoid defining a conflicting `--version` flag.
///
/// # Verbosity and tracing
///
/// Each `-v` or `--verbose` occurrence increments the verbosity count; short flags
/// may be grouped as `-vvv`. With `tracing`, converting owned or borrowed options
/// to `LevelFilter` selects ERROR, WARN, INFO, or DEBUG for verbosity 0, 1, 2, or
/// 3 and above, respectively. `debug` selects TRACE regardless of verbosity.
/// Other flags do not affect that conversion. Without `tracing`, the flags are
/// still parsed and can be interpreted by the application.
///
/// # Examples
///
/// ```
/// use clientele::{crates::clap::{Parser, Subcommand}, StandardOptions};
///
/// #[derive(Parser)]
/// struct Options {
///     #[command(flatten)]
///     flags: StandardOptions,
///     #[command(subcommand)]
///     command: Option<Command>,
/// }
///
/// #[derive(Subcommand)]
/// enum Command { Config }
///
/// let defaults = Options::try_parse_from(["demo"])?;
/// assert!(!defaults.flags.debug);
/// assert!(!defaults.flags.license);
/// assert!(!defaults.flags.version);
/// assert_eq!(defaults.flags.verbose, 0);
///
/// // Global flags work after a subcommand, too.
/// let options = Options::try_parse_from(["demo", "config", "-vv", "--debug"])?;
/// assert_eq!(options.flags.verbose, 2);
/// assert!(options.flags.debug);
///
/// // Command-local flags are accepted only on the containing command.
/// let options = Options::try_parse_from(["demo", "--license", "config"])?;
/// assert!(options.flags.license);
/// assert!(Options::try_parse_from(["demo", "config", "--license"]).is_err());
/// # Ok::<(), clientele::crates::clap::Error>(())
/// ```
#[derive(Debug, Args)]
pub struct StandardOptions {
    #[cfg(feature = "color")]
    /// Set the color output mode
    #[clap(long, default_value_t = clap::ColorChoice::Auto, global = true)]
    pub color: clap::ColorChoice,

    /// Enable debugging output
    #[clap(short = 'd', long, value_parser, global = true)]
    pub debug: bool,

    /// Show license information
    #[clap(long, value_parser)]
    pub license: bool,

    /// Enable verbose output (may be repeated for more verbosity)
    #[clap(short = 'v', long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Print version information
    #[clap(short = 'V', long, value_parser)]
    pub version: bool,
}

/// Converts owned options using the same level mapping as borrowed options.
///
/// Requires `std,clap,tracing`. Also supports `.into()` via the blanket `Into`
/// implementation.
#[cfg(feature = "tracing")]
impl From<StandardOptions> for tracing_core::LevelFilter {
    fn from(options: StandardOptions) -> Self {
        Self::from(&options)
    }
}

/// Selects a tracing level filter without consuming the options.
///
/// Requires `std,clap,tracing`. Verbosity 0, 1, 2, and 3 or higher selects
/// ERROR, WARN, INFO, and DEBUG respectively. `debug` overrides verbosity and
/// selects TRACE. Other flags do not affect the filter. Also supports `.into()`.
#[cfg(feature = "tracing")]
impl From<&StandardOptions> for tracing_core::LevelFilter {
    fn from(options: &StandardOptions) -> Self {
        match (options.debug, options.verbose) {
            (false, 0) => tracing_core::LevelFilter::ERROR,
            (false, 1) => tracing_core::LevelFilter::WARN,
            (false, 2) => tracing_core::LevelFilter::INFO,
            (false, _) => tracing_core::LevelFilter::DEBUG,
            (true, _) => tracing_core::LevelFilter::TRACE,
        }
    }
}

#[cfg(all(test, feature = "tracing"))]
mod tests {
    use super::StandardOptions;
    use tracing_core::LevelFilter;

    #[test]
    fn owned_and_borrowed_level_conversions_preserve_the_mapping() {
        for (verbose, expected) in [
            (0, LevelFilter::ERROR),
            (1, LevelFilter::WARN),
            (2, LevelFilter::INFO),
            (3, LevelFilter::DEBUG),
            (4, LevelFilter::DEBUG),
            (u8::MAX, LevelFilter::DEBUG),
        ] {
            for debug in [false, true] {
                let expected = if debug { LevelFilter::TRACE } else { expected };
                let options = || StandardOptions {
                    #[cfg(feature = "color")]
                    color: clap::ColorChoice::Auto,
                    debug,
                    license: false,
                    verbose,
                    version: false,
                };
                let borrowed = options();
                assert_eq!(LevelFilter::from(&borrowed), expected);
                let borrowed_into: LevelFilter = (&borrowed).into();
                assert_eq!(borrowed_into, expected);
                assert_eq!(LevelFilter::from(options()), expected);
                let owned_into: LevelFilter = options().into();
                assert_eq!(owned_into, expected);
            }
        }
    }
}
