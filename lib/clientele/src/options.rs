// This is free and unencumbered software released into the public domain.

pub mod sort;

extern crate std;

use clap::{ArgAction, Args};

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
