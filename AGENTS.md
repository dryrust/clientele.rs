# Working on Clientele

## Project map
- Work within this project; do not inspect parent directories.
- Rust 2024 workspace; one library, `lib/clientele`, providing CLI utilities and
  dependency re-exports. Declared MSRV: Rust 1.97. Targets: Linux, macOS, Windows.
- `Cargo.toml`: workspace metadata. `lib/clientele/Cargo.toml`: features/dependencies.
- Under `lib/clientele/src/`: `lib.rs` defines exports/module gates; `crates.rs`
  re-exports dependencies; `args.rs` expands arguments; `clap/` and `color.rs`
  handle color/help; `options.rs` defines standard flags; `options/sort.rs` parses
  sort keys, including typed callback parsing; `paths.rs` resolves native and
  environment paths; `subcommands.rs` discovers executables; `tracing.rs`
  configures logging. `completions.rs` and `manpages.rs` expose opt-in generators.
- Tests: inline unit tests and `lib/clientele/tests/`, including subprocess
  drivers for arguments, paths, discovery, logging, and the skeleton CLI.
  Consumer example: `lib/clientele/examples/skeleton/main.rs`.
- `tests/consumer/` is an independent Cargo workspace with its own lockfile:
  it compiles the skeleton and extracted rustdoc examples without direct Clap
  access, and a scoped tracing example without Clap or direct tracing-subscriber.
- `tests/check_features.py` uses Cargo metadata to check every declared feature
  and focused interactions. `tests/version_bump_test.rb` exercises `Rakefile` in
  isolated offline workspaces; `Gemfile.lock` pins its development dependencies.
- [TODO.md](TODO.md): self-contained enhancement backlog with priorities,
  findings, acceptance criteria, and verification commands.

## Editing rules
- Read `TODO.md` before enhancement work. Interpret requests narrowly and work
  on one small, atomic leaf task at a time; remove finished items from `TODO.md`
  after verification.
- Preserve public APIs, CLI behavior, MSRV, and platform-specific semantics.
  Unsafe code is forbidden. Keep optional dependencies optional; update feature
  declarations, module gates, and re-exports together.
- Defaults are `all` + `std`; `all` excludes `error-stack`, `unstable`,
  `completions`, and `manpages`. `clap` and `gofer` enable `std`;
  `error-stack` works with or without `std`.
  Tracing formats need `std,tracing`, their initializer also `clap`.
  Both tracing dependencies are re-exported under `crates` with `tracing`.
  `serde` preserves optional Camino support without pulling in JSON;
  `serde-json` enables Serde and JSON-error integration. `color` and `unicode`
  augment already-enabled dependencies. Generators each enable `clap,std`.
  The standard library is required even with default features disabled.
- Preserve `OsString`/`PathBuf` for OS input; Camino paths explicitly require UTF-8.
  Argument expansion is Windows globs first, then @argfiles; canonical active
  include chains reject cycles while allowing repeated nonrecursive includes.
  Subcommand discovery uses `PATH`, executable permissions on Unix, and `PATHEXT`
  on Windows.
- Add regression tests for behavior changes. Isolate process-global environment
  changes, preferably in subprocesses; existing subcommand fixtures replace `PATH`.
- Document public modules, types, fields, functions, and methods with rustdoc;
  explain feature requirements, defaults, errors/panics, and examples where useful.
  Clap field comments also become CLI help. Prefer rustdoc over README additions;
  expand `README.md` only for essential overview information.
- Record user-visible behavior changes in `CHANGES.md`. For releases, synchronize
  workspace version and `VERSION`; let Cargo update `Cargo.lock`. The Rake task
  increments the patch version using targeted metadata updates, preserves history,
  and restores metadata/lockfile state on failure. After a version bump, refresh
  the independent consumer lockfile with
  `cargo update --manifest-path tests/consumer/Cargo.toml --workspace --offline`.

## Verification
Run relevant checks from the project root:
```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo check -p clientele --all-targets --no-default-features --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked
```
For feature changes, also check affected combinations with
`cargo check -p clientele --all-targets --no-default-features --features <set> --locked`.
Smoke-test CLI changes with `cargo run --locked --example skeleton -- config`.

Additional focused verification:
```sh
python3 tests/check_features.py
python3 tests/check_features.py --toolchain 1.97.0
cargo +1.97.0 test --workspace --all-features --locked
cargo test --manifest-path tests/consumer/Cargo.toml --locked --target-dir target
cargo test --manifest-path tests/consumer/Cargo.toml --all-features --locked --target-dir target
cargo run --manifest-path tests/consumer/Cargo.toml --no-default-features --features tracing --example scoped_tracing --locked --target-dir target
bundle install
bundle exec rake test
```

- CI covers stable/MSRV native tests on Linux, macOS, and Windows; the feature
  driver runs on Linux/Windows using Python 3.13 (the script supports Python 3.9+).
  Release-tooling CI uses Ruby 4.0.6 and the locked bundle.
- Quality CI denies warnings in default/all-feature Clippy and in rustdoc with
  default, all, no default, and Clap-only features. Current checks are warning-free
  without blanket unused-code suppression; retain these gates.
- See `TODO.md` for packaging checks, the dated verification baseline, and open
  platform findings. Cross-compilation does not replace native behavioral tests.

Report check failures; do not silently raise MSRV or disable checks.
