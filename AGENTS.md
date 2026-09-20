# Working on Clientele

## Project map
- Work within this project; do not inspect parent directories.
- Rust 2021 workspace; one library, `lib/clientele`, providing CLI utilities and
  dependency re-exports. Declared MSRV: Rust 1.81. Targets: Linux, macOS, Windows.
- `Cargo.toml`: workspace metadata. `lib/clientele/Cargo.toml`: features/dependencies.
- Under `lib/clientele/src/`: `lib.rs` defines exports/module gates; `crates.rs`
  re-exports dependencies; `args.rs` expands arguments; `clap/` and `color.rs`
  handle color/help; `options.rs` defines standard flags; `options/sort.rs` parses
  sort keys; `paths.rs` resolves environment paths; `subcommands.rs` discovers
  executables; `tracing.rs` configures logging.
- Tests: inline sort tests and `lib/clientele/tests/`. Consumer example:
  `lib/clientele/examples/skeleton/main.rs`.

## Editing rules
- Preserve public APIs, CLI behavior, MSRV, and platform-specific semantics.
  Unsafe code is forbidden. Keep optional dependencies optional; update feature
  declarations, module gates, and re-exports together.
- Defaults are `all` + `std`; `all` excludes `error-stack` and `unstable`.
  `#![no_std]` is commented out: disabling defaults does not prove no-std support.
- Preserve `OsString`/`PathBuf` for OS input; Camino paths explicitly require UTF-8.
  Argument expansion is Windows globs first, then @argfiles. Subcommand discovery
  uses `PATH`, executable permissions on Unix, and `PATHEXT` on Windows.
- Add regression tests for behavior changes. Isolate process-global environment
  changes, preferably in subprocesses; existing subcommand fixtures replace `PATH`.
- Document public modules, types, fields, functions, and methods with rustdoc;
  explain feature requirements, defaults, errors/panics, and examples where useful.
  Clap field comments also become CLI help. Prefer rustdoc over README additions;
  expand `README.md` only for essential overview information.
- Record user-visible behavior changes in `CHANGES.md`. For releases, synchronize
  workspace version and `VERSION`; let Cargo update `Cargo.lock`. The Rake version
  task does broad replacement, including historical changelog entries.

## Verification
Run relevant checks from the project root:
```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo check -p clientele --lib --no-default-features --locked
cargo clippy --workspace --all-targets --locked
cargo doc --workspace --no-deps --locked
```
For feature changes, also check affected combinations with
`cargo check -p clientele --lib --no-default-features --features <set> --locked`.
Smoke-test CLI changes with `cargo run --locked --example skeleton -- config`.

Current baseline caveats (recheck when relevant):
- Minimal all-target builds fail because examples/integration tests lack gates.
- `std,tracing` without `clap` fails; `error-stack` and `--all-features` fail in
  `known-errors`. Clippy and rustdoc have existing warnings.
- Locked dependencies include Rust 1.85 requirements despite the declared 1.81 MSRV.
  Report check failures; do not silently raise MSRV or disable checks.
