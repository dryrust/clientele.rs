// This is free and unencumbered software released into the public domain.

//! Color-policy queries and a pre-parse scan for Clap's help and diagnostics.
//!
//! Requires `clap` (which enables `std`). These APIs are re-exported at the crate
//! root and remain available without `color`. They resolve preferences; callers
//! are responsible for applying the result to their output renderer.

use clap::ColorChoice;
use std::ffi::OsString;

/// The output stream to inspect for automatic color detection.
///
/// Available with the `clap` feature, which also enables `std`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorStream {
    /// Detect whether standard output is a terminal.
    Stdout,
    /// Detect whether standard error is a terminal.
    Stderr,
}

/// Resolves a Clap color choice for an output stream.
///
/// Available with the `clap` feature; querying the policy does not require the
/// `color` feature.
/// Implemented for [`ColorChoice`]. Wrapper types can implement
/// [`Self::as_color_choice`] and inherit the stream-aware default methods.
/// Queries inspect current stream/environment state on each call; they do not
/// change terminal settings or configure Clap or tracing output.
pub trait ColorChoiceExt {
    /// Returns whether color should be enabled for standard output.
    ///
    /// Equivalent to [`Self::to_bool_for`] with [`ColorStream::Stdout`], including
    /// its `NO_COLOR` policy and explicit-choice precedence.
    /// For diagnostics written to stderr, use `to_bool_for(ColorStream::Stderr)`.
    fn to_bool(&self) -> bool {
        self.to_bool_for(ColorStream::Stdout)
    }

    /// Returns whether color should be enabled for the selected output stream.
    ///
    /// [`ColorChoice::Always`] returns `true` and [`ColorChoice::Never`] returns
    /// `false`, regardless of terminal status or environment variables.
    /// [`ColorChoice::Auto`] enables color only when the selected stream is a
    /// terminal and `NO_COLOR` is unset or empty. Any nonempty `NO_COLOR` value,
    /// including `0` or non-UTF-8 values, disables automatic color.
    ///
    /// Terminal detection uses [`std::io::IsTerminal`] and treats detection
    /// failures as non-terminal output. Only `NO_COLOR` is consulted; variables
    /// such as `CLICOLOR`, `CLICOLOR_FORCE`, and `FORCE_COLOR` are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use clientele::{crates::clap::ColorChoice, ColorChoiceExt, ColorStream};
    ///
    /// let choice = ColorChoice::Auto;
    /// let stderr_color = choice.to_bool_for(ColorStream::Stderr);
    ///
    /// assert!(ColorChoice::Always.to_bool_for(ColorStream::Stderr));
    /// assert!(!ColorChoice::Never.to_bool_for(ColorStream::Stdout));
    /// ```
    fn to_bool_for(&self, stream: ColorStream) -> bool {
        use std::{
            env,
            io::{IsTerminal, stderr, stdout},
        };
        color_enabled(
            self.as_color_choice(),
            stream,
            || stdout().is_terminal(),
            || stderr().is_terminal(),
            || env::var_os("NO_COLOR"),
        )
    }

    /// Borrows the underlying Clap color choice.
    ///
    /// Implementors return the policy used by the default query methods. This
    /// method does not perform terminal detection or read environment variables.
    fn as_color_choice(&self) -> &ColorChoice;
}

impl ColorChoiceExt for ColorChoice {
    fn as_color_choice(&self) -> &ColorChoice {
        self
    }
}

// Inject detection so tests can vary each stream and NO_COLOR without changing
// process-global state. Closures preserve short-circuiting for explicit choices.
fn color_enabled(
    choice: &ColorChoice,
    stream: ColorStream,
    stdout_is_terminal: impl FnOnce() -> bool,
    stderr_is_terminal: impl FnOnce() -> bool,
    no_color: impl FnOnce() -> Option<OsString>,
) -> bool {
    match choice {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => {
            let is_terminal = match stream {
                ColorStream::Stdout => stdout_is_terminal(),
                ColorStream::Stderr => stderr_is_terminal(),
            };
            is_terminal && !no_color().is_some_and(|value| !value.is_empty())
        }
    }
}

/// Scans for `--color <when>` / `--color=<when>` ahead of parsing, so that
/// the choice can be fed back into Clap for its own help/usage output.
///
/// Scanning stops at `--`. A separated value is taken only from the immediately
/// following argument, even if it is not valid UTF-8. Invalid or missing values
/// leave the previous choice unchanged, starting from [`ColorChoice::Auto`].
/// The last valid value wins. Only `auto`, `always`, and `never` are accepted,
/// case-sensitively. Unrelated arguments are ignored without converting their
/// native OS representation to UTF-8.
///
/// Requires `clap`, without requiring `color`. This is a best-effort pre-scan,
/// not argument validation: let the full Clap parser report invalid options.
/// It does not inspect terminal status or environment variables. Feed its result
/// to `clap::Command::color` before parsing when `color` is enabled; use
/// [`ColorChoiceExt`] when a renderer instead needs a stream-specific boolean
/// policy.
///
/// ```
/// use clientele::{color_choice, crates::clap::{ColorChoice, Command}};
/// use std::ffi::OsString;
///
/// let args = ["demo", "--color=never"].map(OsString::from);
/// let choice = color_choice(&args);
/// assert_eq!(choice, ColorChoice::Never);
/// #[cfg(feature = "color")]
/// let command = Command::new("demo").color(choice);
/// ```
pub fn color_choice(args: &[OsString]) -> ColorChoice {
    let mut choice = ColorChoice::Auto;
    let mut args = args.iter().take_while(|arg| arg.as_os_str() != "--");
    while let Some(arg) = args.next() {
        let value = if arg == "--color" {
            args.next().and_then(|arg| arg.to_str())
        } else {
            arg.to_str().and_then(|arg| arg.strip_prefix("--color="))
        };
        if let Some(value) = value {
            choice = value.parse().unwrap_or(choice);
        }
    }
    choice
}

#[cfg(test)]
mod tests {
    use super::{ColorChoice, ColorChoiceExt, ColorStream, OsString, color_choice, color_enabled};

    #[test]
    fn auto_detects_each_stream_independently() {
        for (stdout, stderr) in [(false, false), (false, true), (true, false), (true, true)] {
            for (stream, expected) in [(ColorStream::Stdout, stdout), (ColorStream::Stderr, stderr)]
            {
                assert_eq!(
                    color_enabled(&ColorChoice::Auto, stream, || stdout, || stderr, || None),
                    expected,
                    "{stream:?}: stdout terminal={stdout}, stderr terminal={stderr}"
                );
            }
        }
    }

    #[test]
    fn auto_honors_nonempty_no_color() {
        for stream in [ColorStream::Stdout, ColorStream::Stderr] {
            for (no_color, expected) in [
                (None, true),
                (Some(""), true),
                (Some("0"), false),
                (Some("1"), false),
            ] {
                assert_eq!(
                    color_enabled(
                        &ColorChoice::Auto,
                        stream,
                        || true,
                        || true,
                        || no_color.map(OsString::from),
                    ),
                    expected,
                    "{stream:?}: NO_COLOR={no_color:?}"
                );
            }
        }
    }

    #[test]
    fn explicit_choices_override_detection() {
        for stream in [ColorStream::Stdout, ColorStream::Stderr] {
            for (choice, expected) in [(ColorChoice::Always, true), (ColorChoice::Never, false)] {
                assert_eq!(
                    color_enabled(
                        &choice,
                        stream,
                        || panic!("explicit choice must not inspect stdout"),
                        || panic!("explicit choice must not inspect stderr"),
                        || panic!("explicit choice must not inspect NO_COLOR"),
                    ),
                    expected,
                    "{choice:?} on {stream:?}"
                );
            }
        }
    }

    #[test]
    fn to_bool_retains_stdout_semantics() {
        for choice in [ColorChoice::Auto, ColorChoice::Always, ColorChoice::Never] {
            // Trait objects also support the stream-aware method.
            let choice: &dyn ColorChoiceExt = &choice;
            assert_eq!(choice.to_bool(), choice.to_bool_for(ColorStream::Stdout));
        }
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn non_utf8_no_color_disables_auto() {
        #[cfg(unix)]
        let no_color = {
            use std::os::unix::ffi::OsStringExt;
            OsString::from_vec(vec![0xff])
        };
        #[cfg(windows)]
        let no_color = {
            use std::os::windows::ffi::OsStringExt;
            OsString::from_wide(&[0xd800])
        };

        for stream in [ColorStream::Stdout, ColorStream::Stderr] {
            assert!(!color_enabled(
                &ColorChoice::Auto,
                stream,
                || true,
                || true,
                || Some(no_color.clone()),
            ));
        }
    }

    #[test]
    fn stops_at_end_of_options() {
        let args = ["app", "--color=never", "--", "--color=always"].map(OsString::from);
        assert_eq!(color_choice(&args), ColorChoice::Never);

        let args = ["app", "--color", "--", "--color=always"].map(OsString::from);
        assert_eq!(color_choice(&args), ColorChoice::Auto);
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn non_utf8_values_preserve_argument_boundaries() {
        #[cfg(unix)]
        let invalid = {
            use std::os::unix::ffi::OsStringExt;
            OsString::from_vec(vec![0xff])
        };
        #[cfg(windows)]
        let invalid = {
            use std::os::windows::ffi::OsStringExt;
            OsString::from_wide(&[0xd800])
        };

        let mut args = ["app".into(), "--color".into(), invalid, "always".into()];
        assert_eq!(color_choice(&args), ColorChoice::Auto);

        args[3] = "--color=never".into();
        assert_eq!(color_choice(&args), ColorChoice::Never);
    }
}
