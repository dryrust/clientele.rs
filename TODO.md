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

**Suggested next leaf: 2f**, covering hidden subcommand candidates.

Priority: **P1** correctness/reliability, **P2** API/UX/maintenance, **P3** optional
polish or feature growth. Priorities do not override an explicitly selected task.

## 2. Reliable subcommand regression tests (P1)

- [ ] **2f. Cover hidden discovery candidates.**
  Add isolated lookup/listing fixtures for Unix dotfiles and Windows hidden
  attributes. Ensure the prefix matches so the tests exercise hidden-file
  rejection rather than merely rejecting an unrelated filename.
- [ ] **2g. Cover backup-file discovery candidates.**
  Add a matching executable Unix filename ending in `~` and assert exclusion
  from both lookup and complete listing. Keep Windows extension rules explicit.
- [ ] **2h. Cover missing Windows discovery environment.**
  Test unset `PATH` and unset `PATHEXT` independently in child processes with
  matching executable fixtures, asserting empty listing and lookup absence.
  Unix unset-`PATH` coverage already exists in `subcommands_path.rs`.

## 3. Checked sort-to-SQL rendering and invariants (P1)

File: [options/sort.rs](lib/clientele/src/options/sort.rs).

- [ ] **3a. Add checked SQL rendering using approved column mappings.**
  `SortKeys<String>` accepts arbitrary key text and `to_sql()` interpolates it.
  For example, `"(SELECT 1)".parse::<SortKeys>()?.to_sql()` produces
  `(SELECT 1) ASC`. The code already has a SQL-safety TODO. Add an explicit
  checked API, preferably mapping typed enum keys to approved SQL columns, and
  document the trust requirements of the existing raw renderer. Test unknown
  keys, SQL expressions, punctuation/quoting cases, and ascending/descending
  multi-key output. Preserve existing public entry points during the migration.
- [ ] **3b. Specify valid empty/default/whitespace behavior.**
  `SortKeys::<String>::default()` contains one empty key: it displays as an empty
  string but `is_empty()` is false, and SQL rendering yields ` ASC`. Parsing an
  empty string fails, while a whitespace-only key succeeds. Document the intended
  invariants and distinguish `empty()` from a default sort. Test constructors,
  parsing, formatting, and checked rendering; assess compatibility before
  changing existing defaults or accepting/trimming/rejecting whitespace.

## 8. API composability and documentation (P2 unless marked P3)

- [ ] **8a. Add fallible tracing initialization.**
  In `lib/clientele/src/tracing.rs`, add a `try_init_tracing_subscriber()`-style
  API returning initialization errors. Preserve the existing initializer and its
  documented panic behavior. Test successful initialization and a preinstalled
  global subscriber in separate processes. Requirements remain `std,tracing,clap`.
- [ ] **8b. Correct the shared tracing-format representation.**
  `STDERR_PLAIN_FORMAT` and `STDERR_DEBUG_FORMAT` are `const LazyLock` values,
  creating fresh lazy values per use and triggering Clippy warnings. Provide an
  appropriate shared static-backed API. Evaluate compatibility of the public
  constants before replacing their representation; preserve format output and
  availability with `std,tracing` without Clap.
- [ ] **8c. Implement idiomatic log-level conversions.**
  Replace the explicit `Into<LevelFilter>` implementations in
  `lib/clientele/src/options.rs` with `From<StandardOptions>` and
  `From<&StandardOptions>`. Existing `.into()` calls must continue to work.
  Test the current mapping: verbosity 0/1/2/3+ selects ERROR/WARN/INFO/DEBUG;
  `debug` selects TRACE regardless of verbosity.
- [ ] **8d. Add standard subcommand collection interfaces.**
  `SubcommandsProvider` has an inherent `into_iter()` and a `get_commands()`
  returning `&Vec<Subcommand>`. Add owned/borrowed `IntoIterator` support and a
  slice-based accessor while preserving existing methods. Test iteration order,
  borrowing, and ownership; keep discovery behavior changes separate.
- [ ] **8e. Support formatting typed sort keys.**
  `SortKey` and `SortKeys` implement `Display` only for string keys, although Clap
  parsing supports typed `ValueEnum` keys. Generalize formatting for suitable
  `Display` key types. Test a typed key and preserve existing string output,
  direction prefixes, and multi-key separators.
- [ ] **8f. Define robust malformed/non-CSI escape handling.**
  `lib/clientele/src/color.rs::strip_ansi()` is documented for CSI sequences from
  `color_print`; it currently turns `"a\x1bb"` into `"a"` by consuming the next
  character after an unrecognized escape. Specify the handling contract and add
  focused tests for ordinary Unicode, valid CSI, truncated CSI, and non-CSI input.
- [ ] **8g. Extend ANSI stripping to OSC hyperlinks (P3).**
  Depends on the contract in 8f. An OSC hyperlink currently leaves fragments such
  as `8;;https://example.comlabel8;;`. Preserve the visible label while removing
  supported controls; cover BEL and ST terminators and document malformed-input
  behavior. Evaluate any parser dependency against the crate's feature policy.
- [ ] **8h. Complete public rustdoc, one module per change.**
  - [ ] **8h.args:** `lib/clientele/src/args.rs`: features, return values, I/O
    errors, OS-string preservation, Windows glob expansion before @argfiles.
  - [ ] **8h.options:** `lib/clientele/src/options.rs`: standard flag defaults,
    global versus command-local flags, and verbosity behavior. Field docs are
    also Clap help text, so review their CLI effect.
  - [ ] **8h.color:** `lib/clientele/src/clap/color_choice.rs`: the extension
    trait and its methods, stream selection, environment rules, and feature gates.
  - [ ] **8h.sort:** `lib/clientele/src/options/sort.rs`: public constructors and
    accessors, parsing errors, typed-key usage, defaults, and SQL trust contracts.
  - [ ] **8h.paths:** `lib/clientele/src/paths.rs`: feature requirements,
    environment/native resolution, fallback rules, and UTF-8 failure behavior.
  - [ ] **8h.subcommands:** `lib/clientele/src/subcommands.rs`: fields, prefix and
    depth rules, environment/platform behavior, result ordering, and lookup
    naming. Review discoverability of the currently doc-hidden public exports.
- [ ] **8i. Turn ignored examples into executable doctests (P3).**
  - [ ] **8i.help:** Make `lib/clientele/src/clap/help_styles.rs` demonstrate
    `HELP_STYLES` in a complete compilable Clap example.
  - [ ] **8i.sort:** Make the `SortKeys` field snippet in
    `lib/clientele/src/options/sort.rs` a complete compilable parser example.
- [ ] **8j. Fix rustdoc's four bare-URL warnings (P3).**
  Use actual links for the XDG specification references in
  `lib/clientele/src/paths.rs`; verify with `cargo doc`.

## 9. CI, packaging metadata, and release tooling (P2/P3)

CI file: [.github/workflows/ci.yml](.github/workflows/ci.yml). It currently covers
Ubuntu and Windows, minimal feature combinations on Ubuntu, and all-features
tests plus packaged doctests on both systems.

- [ ] **9a. Select Rust toolchains explicitly in CI.**
  Test MSRV 1.97 and current stable instead of relying on the runner's installed
  compiler. Include representative minimal and all-features configurations.
- [ ] **9b. Add native macOS CI coverage.**
  Run behavioral tests on this supported platform, including subprocess fixtures.
- [ ] **9c. Extend critical minimal-feature checks to Windows.**
  Cover supported `clap`, tracing, subcommands, and `error-stack` combinations;
  Windows-specific behavioral tests must run natively.
- [ ] **9d. Add CI quality gates in small steps.**
  - [ ] **9d.fmt:** Add the formatting check.
  - [ ] **9d.doc:** Add rustdoc verification.
  - [ ] **9d.clippy:** Add Clippy verification.
  - [ ] **9d.strict:** Enforce warning-free checks after the relevant baseline
    warnings are resolved. See 8b–8d and 8j; recheck counts before enabling gates.
- [ ] **9e. Use `--locked` consistently in existing CI build/test steps.**
  Feature-matrix and package checks already use it; the default build, example,
  and test steps do not.
- [ ] **9f. Clarify feature documentation (P3).**
  `README.md` calls defaults "all features enabled", but `all` excludes
  `error-stack` and `unstable`. Explain defaults, `all`, `--all-features`, and
  feature prerequisites. Refresh the integration table and placeholder links;
  the tracing conversion is to `LevelFilter`, not the documented `Level`.
  Make the supported `clientele::crates` re-export entry point discoverable.
- [ ] **9g. Align package metadata with actual standard-library support (P3).**
  Root `Cargo.toml` lists the `no-std` category, but `#![no_std]` is commented out
  in `lib.rs`. Correct the claim; a passing no-default-features build alone does
  not establish no-std support.
- [ ] **9h. Document `parse-datetime` as reserved (P3).**
  Its feature definition is empty. Clarify the current status without silently
  removing a public feature or adding an unrequested parser dependency.
- [ ] **9i. Provide a focused CLI dependency recipe (P3).**
  Show `default-features = false` with `clap,dotenv,argfile,wild`, explaining
  opt-in color/logging support. Verify the recipe as a consumer so lightweight
  applications can avoid unrelated runtime/serialization/parser dependencies.
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

For subcommand lookup/listing regressions with only their required features:

```sh
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

For tracing color regressions, including dependency-enabled ANSI without `color`:

```sh
cargo test -p clientele --test tracing_color --no-default-features --features clap,tracing --locked
cargo test -p clientele --test tracing_color --no-default-features --features clap,color,tracing --locked
cargo test -p clientele --test tracing_color --no-default-features --features clap,tracing,tracing-subscriber/ansi --locked
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
tests, and packaged doctests pass. Clippy emits 7 warnings (conversion traits,
the inherent iterator method, and interior-mutable constants); rustdoc emits
4 bare-URL warnings. Earlier runtime probes ran on macOS; Linux/Windows
cross-compilation results are not substitutes for native behavioral tests.
Recheck and report failures rather than suppressing them.

For maintenance of this backlog alone, review file references and run
`git diff --check`; code/test changes require the applicable checks above.
