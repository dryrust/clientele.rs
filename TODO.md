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

**Suggested next leaf: 2a**, asserting lookup presence and absence before
changing subcommand discovery behavior.

Priority: **P1** correctness/reliability, **P2** API/UX/maintenance, **P3** optional
polish or feature growth. Priorities do not override an explicitly selected task.

## 1. Consistent subcommand discovery (P1)

Files: [subcommands.rs](lib/clientele/src/subcommands.rs),
[lookup tests](lib/clientele/tests/subcommands_find.rs), and
[listing tests](lib/clientele/tests/subcommands_list.rs).
Unix examples below were reproduced on macOS. Windows findings came from source
inspection and need native Windows regression tests.

- [ ] **1a. Preserve executable names when collecting commands.**
  With prefix `demo-`, executable `demo-report.v1` is listed as `report` because
  `collect()` uses `file_stem()`. `demo-demo-repeat` becomes `repeat` because
  `trim_start_matches()` removes multiple prefixes. Neither listed name can then
  be resolved by `find()`. Preserve Unix filename suffixes, remove exactly one
  prefix, and handle Windows executable extensions according to platform rules.
  Accept when every fixture's collected name resolves to the same executable;
  use a sufficiently high listing level for names containing additional hyphens.
- [ ] **1b. Make `Subcommand.name` consistent between lookup and listing.**
  `collect("demo-", ...)` returns `hello`, while `find("demo-", "hello")` returns
  `demo-hello`. Define and document the public naming contract, assess existing
  caller compatibility, and test both the returned name and executable path.
- [ ] **1c. Parse `PATHEXT` without indexing unchecked strings.**
  `get_path_exts()` uses `ext[1..]`: empty entries can panic, and malformed entries
  can lose a character or split a Unicode code point. Parse the leading dot
  explicitly and define handling of empty/malformed entries. Test empty values,
  trailing/repeated semicolons, missing dots, mixed case, and non-ASCII input.
  Preserve extension precedence and document behavior when `PATHEXT` is absent.
- [ ] **1d. Reject directories during Windows command discovery.**
  Windows `filter_file()` checks hidden attributes but not `metadata.is_file()`.
  Test that a directory named like `demo-hello.exe` is neither listed nor found,
  while an ordinary matching executable file is accepted.
- [ ] **1e. Honor empty Unix `PATH` components consistently.**
  They denote the current directory. `find()` handles them via `join(command)`,
  but `collect()` attempts `read_dir("")` and skips them. Test empty, leading,
  trailing, and middle components in a subprocess with an isolated working
  directory. Distinguish an unset `PATH` from an empty component.
- [ ] **1f. Make listing deterministic and remove duplicate commands.**
  Repeated `PATH` entries currently duplicate results; directory enumeration
  order is unspecified. Define ordering and logical-name deduplication while
  retaining the first executable selected by `PATH` precedence. Test repeated
  directories and the same command in different directories; listing and lookup
  must agree on the winning executable.

## 2. Reliable subcommand regression tests (P1)

- [ ] **2a. Assert lookup presence and absence unconditionally.**
  In `lib/clientele/tests/subcommands_find.rs`, the presence assertion is
  commented out, so returning `None` for every fixture passes the test. Give
  lookup fixtures explicit expected results. Do not simply reuse
  `should_be_listed`: `clientele-two-levels` is excluded by a shallow listing but
  should still be found by its full subcommand name. The test must fail if
  `find()` always returns `None` or accepts a nonmatching command.
- [ ] **2b. Isolate environment-dependent discovery fixtures.**
  `lib/clientele/tests/subcommands_shared.rs::init()` replaces process-global
  `PATH`. Current binaries each have one test; adding parallel cases requires
  isolation. Follow the child-process pattern in
  `lib/clientele/tests/skeleton_cli.rs`. Set `PATH` and Windows `PATHEXT`
  explicitly in each child so developer configuration cannot affect results.
- [ ] **2c. Assert complete listing results.**
  The listing test searches for expected entries individually, so extra entries
  and duplicates can escape detection. Compare the complete expected result;
  account for the ordering contract selected in 1f. Add non-executable, hidden,
  backup-file, directory, and missing-environment cases as focused follow-ups.
- [ ] **2d. Move shared fixtures out of Cargo's test-target discovery.**
  Move `lib/clientele/tests/subcommands_shared.rs` under a support subdirectory
  and update its importers. Cargo currently runs it as a separate zero-test target.

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

## 7. Environment paths and native directories (P2)

File: [paths.rs](lib/clientele/src/paths.rs). The module requires
`std,getenv,camino`; its public paths are explicitly UTF-8.

- [ ] **7a. Validate XDG home variables as absolute paths.**
  `XDG_CONFIG_HOME=relative/config` currently returns that relative path. The
  [XDG specification](https://specifications.freedesktop.org/basedir-spec/latest/#variables)
  requires relative values to be ignored. Apply validation to data, config,
  state, and cache homes and use their existing home-based fallbacks. Test unset,
  empty, relative, absolute, and unavailable-home cases in isolated processes.
  Empty values already fall back through `getenv`; preserve that behavior.
- [ ] **7b. Add documented OS-native home/temp directory helpers.**
  Current helpers read environment variables; `home()` has a Windows TODO and
  `tmpdir()` returns `None` when `TMPDIR` is unset. Keep those existing contracts
  explicit. Add suitable native-resolution APIs using `dirs`/`std`, preserving
  `PathBuf` for OS paths and using checked conversion for Camino variants. Test
  platform fallbacks and non-UTF-8 handling; keep dependencies feature-gated.

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
  borrowing, and ownership; keep discovery behavior changes in item 1.
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
