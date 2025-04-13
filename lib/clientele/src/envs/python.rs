// This is free and unencumbered software released into the public domain.

use super::var;

pub fn python() -> Option<String> {
    var("PYTHON")
}

/// See: https://docs.python.org/3/using/cmdline.html#envvar-PYTHONHOME
pub fn pythonhome() -> Option<String> {
    var("PYTHONHOME")
}

/// See: https://docs.python.org/3/using/cmdline.html#envvar-PYTHONPATH
pub fn pythonpath() -> Option<String> {
    var("PYTHONPATH")
}

/// See: https://docs.python.org/3/using/cmdline.html#envvar-PYTHONUTF8
/// See: https://peps.python.org/pep-0686/
pub fn pythonutf8() -> Option<bool> {
    var("PYTHONUTF8").map(|s| s == "1")
}

/// See: https://docs.python.org/3/library/venv.html
pub fn virtual_env() -> Option<String> {
    var("VIRTUAL_ENV")
}
