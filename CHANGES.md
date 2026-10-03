# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Added
- `Display` support for typed `SortKey<T>` and `SortKeys<T>` when `T` implements
  `Display`, retaining string output, direction prefixes, and comma separators.
  Constructor calls relying on formatting to infer `String` now need an explicit
  key type, such as `SortKey::<String>::new(...)`
- Owned and borrowed `IntoIterator` support and a `commands()` slice accessor for
  `SubcommandsProvider`, preserving the existing collection methods and ordering
- `From<StandardOptions>` and `From<&StandardOptions>` conversions to tracing
  `LevelFilter`, retaining `.into()` support and the existing verbosity mapping
- `tracing::try_init_tracing_subscriber()` for fallible global subscriber setup,
  preserving the existing initializer's panic behavior
- Document and test empty, default, and whitespace sort semantics, retaining the
  one-key default and verbatim whitespace handling for compatibility
- `SortKey::to_sql_checked()` and `SortKeys::to_sql_checked()` for approved column
  mappings with identifier validation and explicit `SortSqlError` failures
- `ColorChoiceExt::to_bool_for()` and `ColorStream` for stream-aware automatic
  color detection, preserving `to_bool()` as the stdout shorthand
- `paths::home_dir()` and `paths::temp_dir()` for native directory resolution,
  with checked `_utf8` variants and independently gated optional dependencies
### Changed
- `tracing::STDERR_PLAIN_FORMAT` and `STDERR_DEBUG_FORMAT` are shared static
  `LazyLock` values instead of constants. Existing `.clone()` calls still yield
  owned formats with the same output; callers moving or consuming a `LazyLock`
  must instead clone its inner format
- Sort collected subcommands by logical name and return each exact name once,
  selecting executables by `PATH` and Windows lookup/`PATHEXT` precedence
- `SubcommandsProvider::find()` now returns the same prefix-free logical name as
  listing, also omitting the final Windows extension for explicit-name lookups.
  Callers needing the executable filename should use `Subcommand.path.file_name()`
  instead of relying on the previously prefixed `name` field
### Fixed
- Preserve malformed, truncated, and non-CSI escape text in `strip_ansi()`
  instead of dropping characters; remove only complete, syntactically valid CSI
  sequences
- Search the current directory for empty Unix `PATH` components during subcommand
  listing, matching lookup while keeping an unset `PATH` distinct
- Reject directories with executable extensions during Windows subcommand lookup
  and listing
- Ignore empty, bare-dot, and missing-dot `PATHEXT` entries during Windows
  subcommand discovery instead of panicking or truncating malformed input
- Honor `StandardOptions.color` for tracing output using stderr detection, and
  disable ANSI output without `color` even when dependency features enable it
- Ignore relative XDG data, config, state, and cache home overrides and use the
  existing home-based fallbacks
- Preserve Unix filename suffixes and remove only one prefix in collected
  subcommand names; resolve dotted Windows command stems using `PATHEXT`

## 0.4.1 - 2026-10-02
### Fixed
- Require `known-errors` 0.1.2 and use its fixed `error-stack` integration,
  fixing `std,error-stack` and `--all-features` builds
- Stop the color pre-scan at `--` and preserve non-UTF-8 argument boundaries
- Honor the skeleton's `--color` choice for Clap help, errors, and diagnostics
- Resolve the README doctest path through Cargo package metadata so doctests
  work in both the workspace and the packaged crate

## 0.4.0 - 2026-09-21
### Changed
- MSRV is now 1.97 (was 1.81)
### Fixed
- Enabling `clap` now enables its required `std` support
- `std,tracing` builds without `clap`; only the options-based subscriber
  initializer requires `clap`, while tracing formats remain available
- Skip the skeleton example and subcommand integration tests when their required
  features are disabled, allowing minimal all-target builds and tests
- Report `EX_USAGE` (64) instead of panicking when the skeleton receives flags
  without a subcommand
- Preserve sysexits status codes for skeleton application errors, including
  `EX_NOINPUT` (66) for missing @argfiles

## 0.3.14 - 2025-09-04
### Added
- `clientele::options::sort::{SortKey,SortKeys}` methods

## 0.3.13 - 2025-09-04
### Added
- `clientele::tracing::STDERR_{DEBUG,PLAIN}_FORMAT`
- `clientele::tracing::init_tracing_subscriber()`

## 0.3.12 - 2025-09-04
### Changed
- Enable `clientele::options::sort:SortKeys` to use `clap::ValueEnum` types

## 0.3.11 - 2025-09-03
### Added
- `clientele::options::sort::{SortKey,SortKeys}#to_string()`

## 0.3.10 - 2025-09-02
### Added
- `clientele::options::sort::{SortKey,SortKeys}`

## 0.3.9 - 2025-08-26
### Added
- `clientele::HELP_STYLES`
- `clientele::color_choice()`
- `clientele::ColorChoiceExt#to_bool()`
- `clientele::strip_ansi()`

## 0.3.8 - 2025-07-01
### Changed
- Purge the transitive OpenSSL dependency

## 0.3.7 - 2025-06-27
### Added
- Implement `Into<tracing_core::LevelFilter>` for `&StandardOptions`, too

## 0.3.6 - 2025-06-25
### Changed
- Disambiguate `Into<tracing_core::LevelFilter>` for `StandardOptions`

## 0.3.5 - 2025-05-21
### Changed
- Fix a build error with `--no-default-features`

## 0.3.4 - 2025-05-09
### Changed
- Depend on `known-errors` for `sysexits.h` exit codes

## 0.3.3 - 2025-05-05
### Changed
- Depend on and re-export `tracing_core` instead of `tracing`
### Added
- Convert `StandardOptions` to `tracing_core::Level`
- Convert `StandardOptions` to `tracing_core::LevelFilter`

## 0.3.2 - 2025-04-25
### Added
- Extend `SubcommandsProvider`
- Implement more error conversions

## 0.3.1 - 2025-04-25
### Added
- A `gofer` feature that enables integration with Gofer.rs

## 0.3.0 - 2025-04-21
### Changed
- MSRV is now 1.81 (was 1.70)
### Added
- A `getenv` feature that enables integration with Getenv.rs

## 0.2.9 - 2025-04-13
### Added
- Make `envs::var()` public

## 0.2.8 - 2025-04-13
### Added
- More environment variable getters

## 0.2.7 - 2025-04-12
### Added
- `SubcommandsProvider`
- A `serde` feature that enables the optional dependency on Serde
- A `serde-json` feature that enables the optional dependency on serde_json
- Conversion of `serde_json::Error` to `SysexitsError`

## 0.2.6 - 2025-04-12
### Added
- A `tokio` feature that enables the optional dependency on Tokio
- Conversion of `tokio::task::JoinError` to `SysexitsError`

## 0.2.5 - 2025-01-13

## 0.2.4 - 2024-10-15
### Fixed
- Module visibility

## 0.2.3 - 2024-10-15
### Fixed
- Feature flag build

## 0.2.2 - 2024-10-15
### Added
- `clientele::envs`
- `clientele::paths`

## 0.2.1 - 2024-10-13
### Added
- Re-export Camino's `Utf8Path{,Buf}`

## 0.2.0 - 2024-10-01
### Fixed
- Building on Windows

## 0.1.4 - 2024-09-08
### Changed
- `--color` is now a global option

## 0.1.3 - 2024-08-23
### Added
- `--color` standard option
### Changed
- `-v` may now be repeated

## 0.1.2 - 2024-08-23
### Added
- `clientele::StandardOptions`

## 0.1.1 - 2024-08-23
### Added
- More feature flags.

## 0.1.0 - 2024-08-22
### Added
- `clientele::SysexitsError`
- `clientele::SysexitsResult`
- `clientele::abort!()`
- `clientele::args_os()`
- `clientele::exit()`

## 0.0.1 - 2024-08-22
