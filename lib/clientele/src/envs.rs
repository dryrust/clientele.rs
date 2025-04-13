// This is free and unencumbered software released into the public domain.

pub fn home() -> Option<String> {
    var("HOME")
}

pub fn path() -> Option<String> {
    var("PATH")
}

pub fn shell() -> Option<String> {
    var("SHELL")
}

pub fn term() -> Option<String> {
    var("TERM")
}

pub fn tmpdir() -> Option<String> {
    var("TMPDIR")
}

pub fn user() -> Option<String> {
    var("USER")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_data_home() -> Option<String> {
    var("XDG_DATA_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_config_home() -> Option<String> {
    var("XDG_CONFIG_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_state_home() -> Option<String> {
    var("XDG_STATE_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_cache_home() -> Option<String> {
    var("XDG_CACHE_HOME")
}

/// See: https://docs.python.org/3/using/cmdline.html#envvar-PYTHONHOME
pub fn python_home() -> Option<String> {
    var("PYTHONHOME")
}

/// See: https://docs.python.org/3/using/cmdline.html#envvar-PYTHONPATH
pub fn python_path() -> Option<String> {
    var("PYTHONPATH")
}

#[cfg(feature = "std")]
fn var(key: impl AsRef<std::ffi::OsStr>) -> Option<String> {
    use std::env::VarError::*;
    match std::env::var(key) {
        Err(NotPresent | NotUnicode(_)) => None,
        Ok(value) if value.trim().is_empty() => None,
        Ok(value) => Some(String::from(value.trim())),
    }
}

#[cfg(not(feature = "std"))]
fn var(_key: impl AsRef<std::ffi::OsStr>) -> Option<String> {
    None // environment variables not supported
}
