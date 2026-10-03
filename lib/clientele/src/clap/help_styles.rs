// This is free and unencumbered software released into the public domain.

use clap::builder::{styling::AnsiColor, Styles};

/// Help output styling matching the color palette used by Clap v3.
///
/// Available with `clap`. Applying the palette through Clap's `styles` setting
/// requires `color`; the example enables that setting conditionally. Clap's
/// color policy still determines whether ANSI colors are emitted.
///
/// ```
/// use clientele::crates::clap::{CommandFactory, Parser};
///
/// #[derive(Parser)]
/// #[cfg_attr(feature = "color", command(styles = clientele::HELP_STYLES))]
/// struct Options {
///     /// Name to greet
///     #[arg(long)]
///     name: Option<String>,
/// }
///
/// let help = Options::command().render_help().to_string();
/// assert!(help.contains("--name <NAME>"));
/// let options = Options::try_parse_from(["demo", "--name", "World"])?;
/// assert_eq!(options.name.as_deref(), Some("World"));
/// # Ok::<(), clientele::crates::clap::Error>(())
/// ```
pub const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
