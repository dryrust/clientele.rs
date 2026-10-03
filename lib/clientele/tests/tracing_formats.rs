// This is free and unencumbered software released into the public domain.

//! Shared formats work without Clap, using thread-local subscribers for capture.

use clientele::tracing::{STDERR_DEBUG_FORMAT, STDERR_PLAIN_FORMAT};
use std::{
    io::{self, Write},
    sync::{Arc, LazyLock, Mutex},
};
use tracing_subscriber::fmt::format::{Compact, Format};

#[test]
fn formats_share_one_lazy_value_each() {
    for (first, second) in [
        (&STDERR_PLAIN_FORMAT, &STDERR_PLAIN_FORMAT),
        (&STDERR_DEBUG_FORMAT, &STDERR_DEBUG_FORMAT),
    ] {
        assert!(std::ptr::eq(first, second));
        assert!(std::ptr::eq(
            LazyLock::force(first),
            LazyLock::force(second)
        ));
    }
}

#[test]
fn cloned_formats_preserve_output_and_can_be_customized_independently() {
    assert_eq!(capture(STDERR_PLAIN_FORMAT.clone()), "format event\n");
    assert_eq!(
        capture(STDERR_DEBUG_FORMAT.clone()),
        "ERROR clientele_test: format event\n"
    );
    assert_eq!(
        capture(STDERR_PLAIN_FORMAT.clone().with_level(true)),
        "ERROR format event\n"
    );
    assert_eq!(capture(STDERR_PLAIN_FORMAT.clone()), "format event\n");
}

fn capture(format: Format<Compact, ()>) -> String {
    let buffer = Buffer::default();
    let writer = buffer.clone();
    let subscriber = tracing_subscriber::fmt()
        .event_format(format)
        .with_ansi(false)
        .with_writer(move || writer.clone())
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        tracing::error!(target: "clientele_test", "format event");
    });
    let bytes = buffer.0.lock().unwrap().clone();
    String::from_utf8(bytes).expect("UTF-8 tracing output")
}

#[derive(Clone, Default)]
struct Buffer(Arc<Mutex<Vec<u8>>>);

impl Write for Buffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
