// This is free and unencumbered software released into the public domain.

//! Native and environment-derived directory paths.
//!
//! Available with `std`. Native `temp_dir()` needs no additional features;
//! native `home_dir()` also requires `dirs`. Their `_utf8` variants additionally
//! require `camino` and reject paths that cannot be represented as UTF-8.
//!
//! The environment-only `home()`, `tmpdir()`, and XDG helpers require both
//! `getenv` and `camino`. XDG overrides use native absolute-path rules.
//!
//! # Choosing a resolver
//!
//! | Functions | Required features | Resolution |
//! | --- | --- | --- |
//! | `temp_dir` / `temp_dir_utf8` | `std` / `std,camino` | Platform temporary directory |
//! | `home_dir` / `home_dir_utf8` | `std,dirs` / `std,dirs,camino` | Platform home directory |
//! | `home`, `tmpdir` | `std,getenv,camino` | Nonempty UTF-8 environment value only |
//! | `xdg_*_home` | `std,getenv,camino` | Absolute UTF-8 override, then a suffix under `HOME` |
//!
//! Environment values are read on every call without trimming whitespace or
//! expanding `~` or variable references. No helper creates directories, checks
//! permissions, or canonicalizes paths. Native resolvers preserve OS strings;
//! UTF-8 helpers reject unrepresentable paths instead of replacing characters.
//!
//! # XDG fallbacks
//!
//! An accepted XDG override does not require `HOME`. Otherwise the fallback uses
//! the environment-only `home()` resolver, even when `dirs` is enabled. A relative
//! `HOME` therefore produces a relative fallback; an unset, empty, or non-UTF-8
//! `HOME` produces `None`. The suffixes are `.local/share`, `.config`,
//! `.local/state`, and `.cache` for data, configuration, state, and cache.
//!
//! Absolute paths follow the host platform's rules: on Windows a drive-relative
//! path such as `C:cache` or a root-relative path such as `\cache` is not an
//! accepted override, while a fully qualified drive or UNC path is. These
//! helpers use XDG environment conventions on Windows as well as Unix; they do
//! not resolve Windows Known Folders or macOS Library directories.

#[cfg(all(feature = "getenv", feature = "camino"))]
use crate::envs;
#[cfg(feature = "camino")]
use camino::Utf8PathBuf;
use std::path::PathBuf;

/// Returns the user's native home directory, preserving non-UTF-8 OS paths.
///
/// Requires `std,dirs`. Delegates to [`dirs::home_dir`]: on Linux and macOS, a
/// nonempty `HOME` takes precedence over the current user's password-database
/// entry. On Windows, the Known Folder API resolves the user profile directory
/// independently of `HOME`. Returns `None` if no home directory can be found.
/// The returned path is not guaranteed to exist.
///
/// ```
/// if let Some(home) = clientele::paths::home_dir() {
///     let settings = home.join(".my-app");
/// }
/// ```
#[cfg(feature = "dirs")]
pub fn home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

/// Returns the native home directory only if it can be represented as UTF-8.
///
/// Requires `std,dirs,camino`. Uses [`home_dir()`] and a checked conversion to
/// [`Utf8PathBuf`]. Returns `None` when home resolution fails or the path is not
/// UTF-8; no lossy conversion is performed. Use `home_dir()` to retain OS paths.
#[cfg(all(feature = "dirs", feature = "camino"))]
pub fn home_dir_utf8() -> Option<Utf8PathBuf> {
    home_dir().and_then(|path| Utf8PathBuf::from_path_buf(path).ok())
}

/// Returns the native temporary-directory path, preserving non-UTF-8 OS paths.
///
/// Requires only `std`. Delegates to [`std::env::temp_dir`], including its
/// platform-specific environment precedence and fallbacks. On Unix, `TMPDIR`
/// takes precedence over the OS default. On Windows, resolution uses the
/// Windows temporary-path API rather than `TMPDIR`.
///
/// The returned path is not guaranteed to exist. No additional normalization or
/// validation is applied to the platform's result.
///
/// ```
/// let temporary_directory: std::path::PathBuf = clientele::paths::temp_dir();
/// ```
pub fn temp_dir() -> PathBuf {
    std::env::temp_dir()
}

/// Returns the native temporary-directory path only if it is UTF-8.
///
/// Requires `std,camino`. Uses [`temp_dir()`] and a checked conversion to
/// [`Utf8PathBuf`]. Returns `None` if the native path is not UTF-8; no lossy
/// conversion is performed. Use `temp_dir()` to retain OS paths.
#[cfg(feature = "camino")]
pub fn temp_dir_utf8() -> Option<Utf8PathBuf> {
    Utf8PathBuf::from_path_buf(temp_dir()).ok()
}

/// Returns the nonempty UTF-8 value of `HOME`, without native home resolution.
///
/// Requires `std,getenv,camino`. Returns `None` if `HOME` is unset, empty, or not
/// UTF-8. Relative values are returned as-is. This contract also applies on
/// Windows: the user profile is not consulted. For native resolution, use
/// `home_dir()` with the `dirs` feature.
#[cfg(all(feature = "getenv", feature = "camino"))]
pub fn home() -> Option<Utf8PathBuf> {
    envs::home().map(Utf8PathBuf::from)
}

/// Returns the nonempty UTF-8 value of `TMPDIR`, without platform fallbacks.
///
/// Requires `std,getenv,camino`. Returns `None` if `TMPDIR` is unset, empty, or not
/// UTF-8. Relative values are returned as-is, and Windows `TMP`/`TEMP` variables
/// are not consulted. For native resolution, use [`temp_dir()`].
#[cfg(all(feature = "getenv", feature = "camino"))]
pub fn tmpdir() -> Option<Utf8PathBuf> {
    envs::tmpdir().map(Utf8PathBuf::from)
}

/// Returns `XDG_DATA_HOME` if it is an absolute UTF-8 path.
///
/// Requires `std,getenv,camino`.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.local/share` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: <https://specifications.freedesktop.org/basedir-spec/latest/#variables>
#[cfg(all(feature = "getenv", feature = "camino"))]
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
/// Requires `std,getenv,camino`.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.config` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: <https://specifications.freedesktop.org/basedir-spec/latest/#variables>
#[cfg(all(feature = "getenv", feature = "camino"))]
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
/// Requires `std,getenv,camino`.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.local/state` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: <https://specifications.freedesktop.org/basedir-spec/latest/#variables>
#[cfg(all(feature = "getenv", feature = "camino"))]
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
/// Requires `std,getenv,camino`.
///
/// Unset, empty, non-UTF-8, or relative values fall back to `.cache` under
/// [`home()`]. Returns `None` if neither a usable override nor a home directory
/// is available. The directory does not need to exist.
///
/// See: <https://specifications.freedesktop.org/basedir-spec/latest/#variables>
#[cfg(all(feature = "getenv", feature = "camino"))]
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
