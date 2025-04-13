// This is free and unencumbered software released into the public domain.

use super::var;

/// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub fn cargo() -> Option<String> {
    var("CARGO")
}

/// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub fn cargo_home() -> Option<String> {
    var("CARGO_HOME")
}
