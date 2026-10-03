// This is free and unencumbered software released into the public domain.

//! Process arguments with optional wildcard and argument-file expansion.
//!
//! Requires `std`; [`args_os`] is re-exported at the crate root. The `wild` and
//! `argfile` features independently enable the expansion stages described below.

extern crate std;

use std::{ffi::OsString, vec::Vec};

/// Collects the process arguments as owned OS strings, with optional expansion.
///
/// Requires `std`. Returns the complete argument vector, including the first
/// argument (normally the program name), after the enabled expansion stages.
/// Arguments are kept in order, with expansions replacing arguments in place.
/// Pass-through arguments retain their native OS representation, including
/// non-Unicode values; this function does not convert them lossily to UTF-8.
///
/// # Expansion and features
///
/// 1. With `wild`, reads arguments through `wild::args_os()`, expanding unquoted
///    wildcard patterns on Windows. On other platforms this is equivalent to
///    [`std::env::args_os`], leaving wildcard expansion to the invoking shell.
///    Without `wild`, uses `std::env::args_os()` directly on every platform.
/// 2. With `argfile`, replaces `@path` arguments with the contents of UTF-8 files
///    using `argfile`'s Python-style `parse_fromfile` parser. Each line is one
///    argument: spaces and quotes are literal, without shell-style splitting or
///    unquoting. Lines beginning with `@` recursively include another file.
///    Relative paths, including nested includes, resolve from the process's
///    current directory. File paths themselves need not be Unicode. Without
///    `argfile`, `@path` arguments remain literal.
///
/// Windows wildcard expansion runs **before** argument-file expansion. Wildcards
/// introduced by file contents are not expanded by a second wildcard pass.
/// Argument-file expansion examines every argument, including the first one and
/// arguments after `--`; this function does not interpret CLI options.
///
/// # Errors
///
/// With `argfile`, returns an [`std::io::Error`] if any referenced file cannot be
/// opened or read, including missing files, permission errors, and invalid UTF-8
/// contents. No partial argument vector is returned on error. Without `argfile`,
/// this function always returns `Ok` with the collected arguments.
///
/// # Examples
///
/// ```no_run
/// let arguments: Vec<std::ffi::OsString> = clientele::args_os()?;
/// // Skip the program name when processing application arguments directly.
/// for argument in arguments.iter().skip(1) {
///     println!("{argument:?}");
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn args_os() -> Result<Vec<OsString>, std::io::Error> {
    #[cfg(not(feature = "wild"))]
    let args = std::env::args_os();
    #[cfg(feature = "wild")]
    let args = wild::args_os();

    #[cfg(not(feature = "argfile"))]
    return Ok(args.collect());
    #[cfg(feature = "argfile")]
    return argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX);
}
