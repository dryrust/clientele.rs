// This is free and unencumbered software released into the public domain.

use clap::ColorChoice;
use std::ffi::OsString;

pub trait ColorChoiceExt {
    /// Converts this color choice to a boolean value, where `true` means color
    /// should be enabled and `false` means color should be disabled.
    ///
    /// This is used to determine whether color should be enabled for the current
    /// terminal session.
    fn to_bool(&self) -> bool {
        use std::{
            env,
            io::{stdout, IsTerminal},
        };
        match self.as_color_choice() {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => {
                stdout().is_terminal()
                    && !env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty())
            }
        }
    }

    fn as_color_choice(&self) -> &ColorChoice;
}

impl ColorChoiceExt for ColorChoice {
    fn as_color_choice(&self) -> &ColorChoice {
        self
    }
}

/// Scans for `--color <when>` / `--color=<when>` ahead of parsing, so that
/// the choice can be fed back into Clap for its own help/usage output.
///
/// Scanning stops at `--`. A separated value is taken only from the immediately
/// following argument, even if it is not valid UTF-8. Invalid or missing values
/// leave the previous choice unchanged, starting from [`ColorChoice::Auto`].
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
    use super::{color_choice, ColorChoice, OsString};

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
