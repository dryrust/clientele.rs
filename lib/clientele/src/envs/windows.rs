// This is free and unencumbered software released into the public domain.

use super::var;

/// See: https://www.nextofwindows.com/what-is-pathext-environment-variable-in-windows
pub fn pathext() -> Option<String> {
    var("PATHEXT")
}
