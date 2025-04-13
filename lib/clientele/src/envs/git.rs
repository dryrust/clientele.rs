// This is free and unencumbered software released into the public domain.

use super::var;

/// See: https://git-scm.com/book/en/v2/Git-Internals-Environment-Variables
pub fn git() -> Option<String> {
    var("GIT")
}

/// See: https://git-scm.com/book/en/v2/Git-Internals-Environment-Variables
pub fn git_dir() -> Option<String> {
    var("GIT_DIR")
}
