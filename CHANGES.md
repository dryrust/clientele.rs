# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
