// This is free and unencumbered software released into the public domain.

use crate::StandardOptions;
use std::sync::LazyLock;
use tracing_subscriber::fmt::{
    format::{Compact, Format},
    time::SystemTime,
};

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

pub const STDERR_DEBUG_FORMAT: LazyLock<Format<Compact, ()>> =
    LazyLock::new(|| tracing_subscriber::fmt::format().compact().without_time());

/// Initializes `tracing_subscriber` based on the given options.
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
