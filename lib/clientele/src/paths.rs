// This is free and unencumbered software released into the public domain.

//! UTF-8 paths resolved from environment variables.
//!
//! Available with the `std`, `getenv`, and `camino` features. XDG home overrides
//! use the platform's native absolute-path rules.

use crate::envs;
use camino::Utf8PathBuf;

pub fn home() -> Option<Utf8PathBuf> {
    envs::home().map(Utf8PathBuf::from) // TODO: Windows
}

pub fn tmpdir() -> Option<Utf8PathBuf> {
    envs::tmpdir().map(Utf8PathBuf::from)
}

/// Returns `XDG_DATA_HOME` if it is an absolute UTF-8 path.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.local/share` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_data_home() -> Option<Utf8PathBuf> {
    envs::xdg_data_home()
        .map(Utf8PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| {
            home().map(|mut path| {
                path.push(".local/share");
                path
            })
        })
}

/// Returns `XDG_CONFIG_HOME` if it is an absolute UTF-8 path.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.config` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_config_home() -> Option<Utf8PathBuf> {
    envs::xdg_config_home()
        .map(Utf8PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| {
            home().map(|mut path| {
                path.push(".config");
                path
            })
        })
}

/// Returns `XDG_STATE_HOME` if it is an absolute UTF-8 path.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.local/state` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_state_home() -> Option<Utf8PathBuf> {
    envs::xdg_state_home()
        .map(Utf8PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| {
            home().map(|mut path| {
                path.push(".local/state");
                path
            })
        })
}

/// Returns `XDG_CACHE_HOME` if it is an absolute UTF-8 path.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.cache` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_cache_home() -> Option<Utf8PathBuf> {
    envs::xdg_cache_home()
        .map(Utf8PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| {
            home().map(|mut path| {
                path.push(".cache");
                path
            })
        })
}
