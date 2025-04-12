# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
