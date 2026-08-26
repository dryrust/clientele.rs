// This is free and unencumbered software released into the public domain.

use clap::builder::{styling::AnsiColor, Styles};

/// Help output styling matching the color palette used by Clap v3.
///
/// ```ignore
/// #[command(styles = clientele::HELP_STYLES)]
/// ```
pub const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
