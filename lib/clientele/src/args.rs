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
///    Canonical paths track the active include chain: including a file already
///    in that chain is an error, including through relative or symbolic-link
///    aliases. A file may be included again after its earlier expansion finishes.
///
/// Windows wildcard expansion runs **before** argument-file expansion. Wildcards
/// introduced by file contents are not expanded by a second wildcard pass.
/// Argument-file expansion examines every argument, including the first one and
/// arguments after `--`; this function does not interpret CLI options.
///
/// # Errors
///
/// With `argfile`, returns an [`std::io::Error`] if any referenced file cannot be
/// resolved, opened, or read, including missing files, permission errors, and
/// invalid UTF-8 contents. Recursive inclusion returns
/// [`std::io::ErrorKind::InvalidInput`]. No partial argument vector is returned on
/// error. Without `argfile`, this function always returns `Ok` with the collected
/// arguments.
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
    return expand_argfiles(args);
}

#[cfg(feature = "argfile")]
fn expand_argfiles(args: impl Iterator<Item = OsString>) -> std::io::Result<Vec<OsString>> {
    use argfile::{Argument, PREFIX};
    use std::{collections::HashSet, fs, io, path::PathBuf};

    enum Work {
        Argument(Argument),
        EndFile(PathBuf),
    }

    let mut pending: Vec<_> = args
        .map(|arg| Work::Argument(Argument::parse(arg, PREFIX)))
        .collect();
    let mut expanded = Vec::with_capacity(pending.len());
    pending.reverse();
    let mut active = HashSet::new();

    // An explicit work stack avoids consuming the call stack for nested files.
    while let Some(work) = pending.pop() {
        match work {
            Work::EndFile(path) => {
                active.remove(&path);
            }
            Work::Argument(Argument::PassThrough(arg)) => expanded.push(arg),
            Work::Argument(Argument::Path(path)) => {
                let with_path = |error: io::Error| {
                    io::Error::new(error.kind(), format!("argument file {path:?}: {error}"))
                };
                let canonical = fs::canonicalize(&path).map_err(with_path)?;
                if !active.insert(canonical.clone()) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("recursive argument file inclusion: {path:?}"),
                    ));
                }
                let content = fs::read_to_string(&path).map_err(with_path)?;
                pending.push(Work::EndFile(canonical));
                pending.extend(
                    argfile::parse_fromfile(&content, PREFIX)
                        .into_iter()
                        .rev()
                        .map(Work::Argument),
                );
            }
        }
    }
    Ok(expanded)
}
