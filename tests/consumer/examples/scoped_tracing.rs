use clientele::{crates::tracing_subscriber, tracing::STDERR_PLAIN_FORMAT};

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .event_format(STDERR_PLAIN_FORMAT.clone())
        .with_ansi(false)
        .with_writer(std::io::sink)
        .finish();
    tracing::subscriber::with_default(subscriber, || tracing::info!("scoped event"));
}
