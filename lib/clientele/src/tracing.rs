// This is free and unencumbered software released into the public domain.

use std::sync::LazyLock;
use tracing_subscriber::fmt::format::{Compact, Format};

pub const STDERR_FORMAT: LazyLock<Format<Compact, ()>> = LazyLock::new(|| {
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
