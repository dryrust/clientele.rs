# Clientele enhancement backlog

Note: the next release is going to be 0.5.0, meaning backwards
incompatibility does not need to be strictly preserved.

Review snapshot: 2026-10-02. This file records the outstanding project review
findings and enough context to continue without the original conversation.
Task IDs are stable and may contain gaps. Recheck the relevant code before
implementing a task.

## Working method

- Follow [AGENTS.md](AGENTS.md). This is a Rust 2021 library workspace with MSRV
  1.97 and Linux, macOS, and Windows targets.
- Work in small, atomic changes: choose **one unchecked leaf item**, interpret
  the request narrowly, and keep section headings as groups of independent
  changes. For behavior changes, include meaningful regression coverage,
  rustdoc, and a user-visible changelog entry.
- Preserve public APIs and platform semantics. Prefer additive APIs where changing
  an existing signature or behavior would break callers. Record compatibility
  decisions for observable changes such as command names and sorting defaults.
- Isolate environment and global-subscriber tests in subprocesses. Keep optional
  dependencies optional. Only create commits when explicitly requested.
- After verification, remove finished items from this backlog. Keep the IDs of
  remaining tasks stable so later requests can refer to them.

**Suggested next leaf: 9k**, documenting skeleton usage.

Priority: **P1** correctness/reliability, **P2** API/UX/maintenance, **P3** optional
polish or feature growth. Priorities do not override an explicitly selected task.

## 9. CI, packaging metadata, and release tooling (P2/P3)

CI file: [.github/workflows/ci.yml](.github/workflows/ci.yml). It currently covers
Ubuntu, macOS, and Windows with default and all-features tests on Rust 1.97.0
and stable. Ubuntu and Windows also run minimal feature combinations and
packaged doctests.

- [ ] **9d. Add CI quality gates in small steps.**
  - [ ] **9d.strict:** Enforce warning-free checks after the relevant baseline
    warnings are resolved. The compatibility-preserved inherent
    `SubcommandsProvider::into_iter()` still triggers `should_implement_trait`
    despite owned/borrowed `IntoIterator` implementations; assess its signature
    compatibility before changing it. Recheck counts before enabling gates.
- [ ] **9j. Replace broad version replacement in `Rakefile`.**
  `version:bump` performs repository-wide replacement of the old version and can
  rewrite historical changelog entries. Update intended metadata explicitly,
  synchronize root `Cargo.toml` and `VERSION`, and let Cargo update `Cargo.lock`.
  Test in an isolated fixture that historical release entries remain intact.
- [ ] **9k. Document skeleton usage (P3).**
  `lib/clientele/examples/skeleton/README.md` is empty. Explain its initialization
  sequence, available features, and exit statuses: Clap errors 2, missing
  subcommand after flags 64, missing @argfile 66, help/version/license 0. Keep
  the instructions aligned with its existing subprocess tests.
- [ ] **9l. Demonstrate logging initialization in the skeleton (P3).**
  After the relevant tracing/color APIs are settled, show their use behind the
  `tracing` feature and retain builds with only `clap,dotenv`. Treat this as a
  separate example change from 6b's help/diagnostic color correction.

## 10. Optional CLI feature growth (P3)

- [ ] **10a. Add opt-in shell completion generation.**
  Evaluate `clap_complete`, keep the dependency optional, document the feature and
  consumer usage, and test generated output against a representative command.
- [ ] **10b. Add opt-in man-page generation.**
  Evaluate `clap_mangen` in a separate change with the same feature, documentation,
  and consumer-verification discipline.

## Verification and baseline

Run relevant checks from the repository root as required by `AGENTS.md`:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo check -p clientele --all-targets --no-default-features --locked
cargo clippy --workspace --all-targets --locked
cargo doc --workspace --no-deps --locked
```

Add focused tests for the selected leaf. For feature changes, check affected
combinations with `cargo check -p clientele --all-targets --no-default-features
--features <set> --locked`, replacing `<set>` with the applicable feature list.
Useful sets: `clap`; `std,subcommands`; `std,getenv,camino`; `std,tracing`;
`clap,tracing`; `clap,color,tracing`; `error-stack`; `std,error-stack`.
Preserve optional-example/test gates in `lib/clientele/Cargo.toml`.

For ANSI stripping and its doctests without optional features:

```sh
cargo test -p clientele --no-default-features --locked
```

For sort parsing, checked SQL rendering, and their doctests with minimal features:

```sh
cargo test -p clientele --no-default-features --features clap --locked
```

For subcommand collection interfaces and lookup/listing regressions with only
their required features:

```sh
cargo test -p clientele --lib --no-default-features --features std,subcommands --locked
cargo test -p clientele --test subcommands_find --test subcommands_list --no-default-features --features std,subcommands --locked
cargo test -p clientele --test subcommands_path --no-default-features --features std,subcommands --locked
cargo test -p clientele --test subcommands_order --no-default-features --features std,subcommands --locked
```

For native directory resolution, checked UTF-8 conversion, and their feature gates:

```sh
cargo test -p clientele --test paths_native --no-default-features --features std --locked
cargo test -p clientele --no-default-features --features std,dirs,camino --locked
```

For XDG environment-path regressions:

```sh
cargo test -p clientele --test paths_xdg --no-default-features --features std,getenv,camino --locked
```

For skeleton color regressions, run the existing CLI driver in both configurations:

```sh
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,dotenv --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,color,dotenv --locked
cargo run --locked --example skeleton -- config
```

For shared tracing formats and their doctests without Clap:

```sh
cargo test -p clientele --no-default-features --features std,tracing --locked
```

For tracing initialization and color regressions, including dependency-enabled
ANSI without `color`:

```sh
cargo test -p clientele --test tracing_init --test tracing_color --no-default-features --features clap,tracing --locked
cargo test -p clientele --test tracing_init --test tracing_color --no-default-features --features clap,color,tracing --locked
cargo test -p clientele --test tracing_init --test tracing_color --no-default-features --features clap,tracing,tracing-subscriber/ansi --locked
```

For release/dependency checks, retain MSRV and all-features coverage:

```sh
cargo +1.97.0 test --workspace --all-features --locked
```

For README/packaging changes, reproduce CI's packaged-doctest check. The local
`--allow-dirty` flag permits verification of uncommitted edits:

```sh
cargo package -p clientele --locked --allow-dirty --target-dir target
for manifest in target/package/clientele-*/Cargo.toml; do
  cargo test --manifest-path "$manifest" --target-dir target --doc --locked
  cargo test --manifest-path "$manifest" --target-dir target --doc --no-default-features --locked
done
```

Last observed baseline: default tests, relevant minimal builds, all-features
tests, and packaged doctests pass. Clippy emits 1 warning (the inherent iterator
method); rustdoc is warning-free. Earlier runtime probes ran on macOS; Linux/Windows
cross-compilation results are not substitutes for native behavioral tests.
Recheck and report failures rather than suppressing them.

For maintenance of this backlog alone, review file references and run
`git diff --check`; code/test changes require the applicable checks above.
