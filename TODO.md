# Clientele enhancement backlog

Note: the next release is going to be 0.5.0, meaning backwards
incompatibility does not need to be strictly preserved.

Review snapshot: 2026-10-03. This file records the outstanding project review
findings and enough context to continue without the original conversation.
Task IDs are stable and may contain gaps. Recheck the relevant code before
implementing a task. The `R2-*` IDs identify findings from this review; completed
items from the previous review have not been reopened.

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

**Status:** 20 outstanding leaf tasks: 4 P1, 14 P2, and 2 P3. Evidence below
distinguishes runtime reproductions, source-review findings, coverage gaps, and
optional extensions.

Priority: **P1** correctness/reliability, **P2** API/UX/maintenance, **P3** optional
polish or feature growth. Priorities do not override an explicitly selected task.

## Review coverage

Reviewed all tracked source modules, integration/unit tests, the skeleton example,
manifests and lockfile, documentation, and CI/release/developer tooling. Behavioral
probes ran on macOS; Windows-specific findings below still need native regression
tests. Cross-compilation establishes build coverage only.

| Area | Review outcome |
| --- | --- |
| Feature gates, dependency re-exports, consumer builds | Unexpected JSON dependency with `serde`; downstream derive examples fail; R2-07, R2-10, R2-11 |
| Arguments and skeleton CLI | Lost error context, broken-pipe panic, expansion coverage gaps; R2-08, R2-09, R2-12, R2-13 |
| Executable discovery | Case-sensitive-prefix mismatch reproduced on macOS; Windows extension/identity findings and OS-path coverage gaps; R2-03 through R2-05, R2-16, R2-22 |
| Color scanning, ANSI/OSC stripping, sort parsing and checked SQL | Existing implementations and regression tests reviewed; optional typed-parser reuse in R2-21 |
| Native/UTF-8/XDG paths and tracing | Existing path, format, color, and global-initialization tests pass locally; dependency access and isolated-feature coverage in R2-11, R2-14 |
| Completions, manpages, error-stack, packaging | All-feature MSRV tests and packaged default/minimal doctests pass; isolated-feature and quality gates in R2-14, R2-18 |
| CI, Rake, Make, project documentation | Version-bump rollback failure; missing Ruby CI coverage, suppressed unused warnings, stale guidance/credits; R2-06, R2-15, R2-17, R2-19, R2-20 |

## P1 — Correctness and reliability

- [ ] **R2-03 — Reject path separators in Windows `PATHEXT` entries.**
  **Evidence (source review plus primitive probe):** In
  `lib/clientele/src/subcommands.rs`, `parse_path_exts` accepts `.bad/name` and
  `.bad\name`. `resolve_command` passes the suffix to `Path::with_added_extension`,
  which panics on a host path separator. The slash panic was reproduced on macOS;
  both separators need native Windows coverage. A malformed entry before `.BAT`
  can also affect collection's lookup pass.
  **Acceptance:** Ignore separator-containing entries before path construction;
  retain the documented precedence, Unicode, and non-trimming behavior for valid
  entries. Test malformed-only and malformed-before-valid lists through both
  `find` and `collect`, as well as the platform-independent parser tests.
  **Verify:** `cargo test -p clientele --lib --no-default-features --features std,subcommands --locked`;
  extend and run `subcommands_path` on Windows with the same feature set.

- [ ] **R2-04 — Reconcile Windows dotted-name collection and lookup identity.**
  **Evidence (source review):** In `lib/clientele/src/subcommands.rs`, Windows
  `collect` derives a name, then replaces only `command.path` through
  `resolve_command`. With `PATHEXT=.BAT` and both
  `demo-report.v1` and `demo-report.v1.bat` present, collection can retain the name
  `report.v1` but select the exact file `demo-report.v1`. `find("demo-",
  "report.v1")` then derives `report` from that same path. This violates the
  documented shared naming rules and the round-trip expectation in
  `lib/clientele/tests/subcommands_list.rs`.
  **Acceptance:** Define and implement a consistent policy for the ambiguity
  between a dotted logical stem and an explicit filename. Every collected entry
  must round-trip through lookup with the same name and path; retain deterministic
  ordering/deduplication and record any change to exact-file precedence. Test the
  collision in one directory and across differently ordered `PATH` directories.
  **Verify:** Native Windows runs of `subcommands_find`, `subcommands_list`, and
  `subcommands_order`, minimally with `std,subcommands` and with defaults.

- [ ] **R2-05 — Enforce consistent prefix matching on case-insensitive filesystems.**
  **Evidence (reproduced):** On the review machine's case-insensitive macOS
  filesystem, an executable named `DEMO-hello` yielded no entries from
  `collect("demo-", 1)`, while `find("demo-", "hello")` returned it. In
  `lib/clientele/src/subcommands.rs`, listing checks directory-entry spelling,
  but lookup checks the constructed candidate path's spelling rather than the
  actual filename.
  The public contract says prefix matching is literal and case-sensitive.
  **Acceptance:** Make both operations apply the same prefix policy to actual
  filenames. Preserve the documented case-sensitive policy unless an explicit
  compatibility decision replaces it. Include case-colliding candidates in
  different `PATH` directories and verify returned logical names/path precedence.
  **Verify:** Extend the discovery subprocess tests; run on native Windows and
  case-insensitive macOS, with a case-sensitive-filesystem control where available.

- [ ] **R2-06 — Roll back a version bump when Cargo fails.**
  **Evidence (reproduced):** `Rakefile` writes `Cargo.toml` and `VERSION` before
  running `cargo update --workspace --offline`. In an isolated workspace with a
  missing member, the task failed but advanced both files from 0.4.1 to 0.4.2,
  leaving `Cargo.lock` at 0.4.1. Existing `tests/version_bump_test.rb` covers
  validation failures before writing, not failure of the Cargo operation.
  **Acceptance:** Restore the original metadata and lockfile state on Cargo
  failure, including an initially absent lockfile, and return failure to Rake.
  Add a deterministic failure fixture and ensure a retry consumes only one patch
  version. Preserve historical text and unrelated locked dependency versions.
  **Verify:** `rake test`, including the new rollback regression and the existing
  successful-bump/validation cases; do not exercise failure against the real workspace.

## P2 — Public API and CLI usability

- [ ] **R2-07 — Make derive examples compile in a downstream-only consumer.**
  **Evidence (reproduced):** Copying the `StandardOptions` rustdoc example into
  a standalone crate depending only on Clientele fails with E0433, unresolved
  `clap`. Importing `clientele::crates::clap::{self, Parser, Subcommand}` fixes it.
  The same missing module import occurs in `src/clap/help_styles.rs`,
  `src/options/sort.rs`, `src/tracing.rs`, and `examples/skeleton/main.rs` under
  `lib/clientele/`. In-package doctests/examples can see the direct Clap dependency,
  masking this problem. README and generator examples already use `self` correctly.
  **Acceptance:** Fix the affected imports and add a standalone consumer fixture
  that cannot obtain Clap through Clientele's package extern prelude. Verify the
  documented minimal features; the copied skeleton may add its documented direct
  `tracing` dependency for event macros, but must not need a direct Clap dependency.
  **Verify:** `cargo test -p clientele --doc --no-default-features --features clap --locked`,
  plus `cargo check --manifest-path <consumer-fixture>/Cargo.toml` outside the
  library package and checks for the copied skeleton's required feature set.

- [ ] **R2-08 — Preserve source error context in skeleton diagnostics.**
  **Evidence (reproduced):** A missing `@missing-args.txt` prints only
  `Error: EX_NOINPUT` and exits 66. In
  `lib/clientele/examples/skeleton/main.rs::run`, conversion by `?` to
  `SysexitsError` discards the `io::Error` containing the filename and cause. The
  existing `skeleton_cli` test currently accepts that context-free output.
  **Acceptance:** Retain the underlying error for display while preserving its
  sysexits status. Diagnostics should identify the failed operation/file and cause.
  Keep the example usable with its current minimal features. Cover missing and
  invalid-UTF-8 argument files without depending on exact OS error wording.
  **Verify:** `cargo test -p clientele --test skeleton_cli --locked` and the same
  target with `--no-default-features --features clap,dotenv,argfile --locked`.

- [ ] **R2-09 — Handle output errors without panicking in the skeleton.**
  **Evidence (reproduced):** Running `skeleton config` with stdout connected to a
  pipe whose read end was already closed exits 101 with `failed printing to stdout:
  Broken pipe`. Its `println!` calls bypass the fallible application error path;
  the final `eprintln!` can also panic when reporting another output failure.
  **Acceptance:** Use fallible writes, document the CLI's broken-pipe exit policy,
  and avoid a panic when either output stream fails. Preserve ordinary help,
  version/license, config output, and application error statuses. Use a controlled
  closed pipe in a subprocess test rather than a timing-sensitive shell pipeline.
  **Verify:** The `skeleton_cli` target with defaults and with
  `--no-default-features --features clap,dotenv --locked`, plus the closed-pipe case.

- [ ] **R2-10 — Keep JSON integration out of a Serde-only dependency selection.**
  **Evidence (dependency graph):** `serde = ["dep:serde", "known-errors/serde",
  "camino?/serde1"]` in `lib/clientele/Cargo.toml` enables `serde_json` through
  `known-errors/serde -> known-errors/serde-json`, even when Clientele's
  `serde-json` feature is disabled. The separate JSON feature already forwards
  `known-errors/serde-json` explicitly.
  **Acceptance:** Make Serde-only selection omit JSON unless another enabled
  dependency requires it. Preserve Camino serialization and the explicit
  `serde-json` integration. Record the compatibility impact on JSON-error
  conversions previously enabled transitively by `serde` alone.
  **Verify:** `cargo tree -p clientele --no-default-features --features serde -e normal --locked`;
  check `serde`, `serde,camino`, `serde-json`, and `std,serde-json` independently.

- [ ] **R2-11 — Expose the tracing formatter dependency through `crates`.**
  **Evidence (API review):** `lib/clientele/src/tracing.rs` exposes types from
  `tracing_subscriber`, but `lib/clientele/src/crates.rs` re-exports only
  `tracing_core` under `tracing`. Consumers naming those format types or composing
  their own subscriber must introduce another direct dependency despite the
  documented dependency sharing entry point.
  **Acceptance:** Add the feature-gated `tracing_subscriber` re-export and document
  its feature-name mapping. Show a downstream consumer using the shared formats
  with a locally scoped/custom subscriber through that re-export.
  **Verify:** A standalone consumer with only `std,tracing`; rustdoc and
  `cargo check -p clientele --all-targets --no-default-features --features std,tracing --locked`.

## P2 — Regression coverage and automation

- [ ] **R2-12 — Test the argument-file and OS-string contracts directly.**
  **Evidence (coverage gap):** `lib/clientele/tests/args_os.rs` now covers cycles,
  repeated nested includes, quoted line contents, and OS-string arguments, but
  does not yet cover invalid UTF-8 file contents, expansion after `--`, or argv[0].
  Native non-Unicode filename fixtures run on Linux/Windows because macOS rejects
  those filenames. `lib/clientele/tests/skeleton_cli.rs` uses string-only arguments.
  **Acceptance:** Extend the harness-free `args_os` target with the remaining
  documented contracts, including errors without partial results and argument
  ordering. Retain disabled-feature pass-through and recursion coverage, and
  keep environment changes inside child processes.
  **Verify:** Run the target with defaults and with each of `std` and
  `std,argfile` under `--no-default-features --locked`.

- [ ] **R2-13 — Exercise Windows wildcard expansion with real raw command lines.**
  **Evidence (coverage gap):** No existing test exercises `wild::args_os` with a
  wildcard. Passing ordinary strings through `Command::args` alone does not prove
  quoted-versus-unquoted Windows command-line behavior or its ordering with argfiles.
  **Acceptance:** Add a native Windows subprocess target, gated by `std,wild`,
  with controlled files and raw/quoted wildcard arguments. Cover matching and
  unmatched patterns, spaces in filenames, literal quoted patterns, and the rule
  that wildcards introduced by argfile contents are not expanded again. Avoid
  depending on the developer's shell or directory contents.
  **Verify:** Native Windows tests with `--no-default-features --features std,wild --locked`
  and `--no-default-features --features std,wild,argfile --locked`, plus defaults.

- [ ] **R2-14 — Cover every standalone feature and important weak-feature combination in CI.**
  **Evidence (coverage gap):** `.github/workflows/ci.yml` tests a curated minimal
  list, but omits standalone `completions` and `manpages`, among others.
  Default/all-feature builds cannot establish independent generator gates or
  optional dependency behavior. Path combinations documented below are
  also absent from the current minimal loops.
  **Acceptance:** Check every public feature independently with
  defaults disabled on stable/MSRV. Include library-only builds to avoid
  dev-dependency feature unification, relevant all-target builds, and focused tests
  for `completions`, `manpages`, `std,dirs,camino`, `std,getenv,camino`, and tracing
  ANSI enabled through the dependency without Clientele's `color`. Retain native
  platform testing for platform behavior; keep the matrix focused rather than
  attempting every possible feature power set.
  **Verify:** Reproduce each added matrix command locally where supported and
  require the corresponding CI jobs to pass on their native runners.

- [ ] **R2-15 — Run release-tooling regressions in CI.**
  **Evidence (coverage gap):** `rake test` currently passes its two tests and
  30 assertions, but no workflow step runs `tests/version_bump_test.rb`. Rust tests
  and packaging checks cannot detect regressions in `Rakefile`.
  **Acceptance:** Set up reproducible Ruby, Rake, and Minitest dependencies in one
  CI job and execute `rake test`. Keep fixtures isolated/offline and surface failures
  as job failures. The rollback test from R2-06 must run automatically once added.
  **Verify:** `rake test` and the new CI job from a clean checkout.

- [ ] **R2-16 — Cover non-Unicode paths in executable discovery.**
  **Evidence (coverage gap):** The discovery contract promises to preserve
  non-UTF-8 parent directories while skipping non-UTF-8 filenames, but the
  fixtures in `lib/clientele/tests/support/subcommands_shared.rs` and the four
  discovery targets use only UTF-8 paths. Color/native-path tests do not cover
  discovery.
  **Acceptance:** Add isolated fixtures using platform-native non-Unicode path
  components where the filesystem supports them. Assert that both lookup and
  listing preserve the exact `PathBuf` through a non-Unicode `PATH` directory,
  ignore unrepresentable executable names, and agree on valid neighboring entries.
  **Verify:** Native Unix and Windows discovery tests with `std,subcommands`
  alone; compare OS paths directly rather than their lossy display text.

- [ ] **R2-17 — Replace blanket unused-code suppression with precise gates.**
  **Evidence (reproduced):** `#![allow(unused)]` in `lib/clientele/src/lib.rs`,
  `lib/clientele/src/prelude.rs`, and the skeleton weakens the otherwise
  warning-denying checks. A forced-warning build reports unused prelude exports, redundant
  `extern crate std` declarations, and `time::SystemTime` in
  `lib/clientele/src/tracing.rs`.
  **Acceptance:** Remove obsolete imports/private scaffolding and blanket
  allowances; use narrowly justified feature gates or local allowances only where
  needed. Keep the documented standard-library requirement and all feature builds.
  **Verify:** `cargo rustc -p clientele --lib --all-features --locked -- --force-warn unused`,
  default/minimal/all-feature Clippy, and affected example builds.

- [ ] **R2-18 — Apply warning-denying quality gates to opt-in APIs.**
  **Evidence (CI review):** The quality job runs Clippy and rustdoc with defaults
  only. `completions`, `manpages`, and `error-stack` are excluded from those gates;
  all-feature tests do not replace rustdoc link checking or Clippy. All-feature
  Clippy/rustdoc checks pass in this review, so this is prevention rather than a
  currently failing baseline.
  **Acceptance:** Add all-feature Clippy/rustdoc coverage and a small
  minimal-feature documentation check to CI, retaining `-D warnings` and existing gates.
  **Verify:** `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`;
  `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps --locked`;
  minimal rustdoc with `--no-default-features` and `clap` alone.

## P2 — Project documentation

- [ ] **R2-19 — Refresh contributor guidance to match the current project.**
  **Evidence (documentation review):** `AGENTS.md` still says the Rake version
  task broadly replaces historical versions and that Clippy/rustdoc have existing
  warnings. The task now updates specific metadata, and ordinary warning-denying
  quality checks pass. Its project map/bundle description also omits the new
  generator modules, opt-in exclusions, and Ruby release tests.
  **Acceptance:** Correct these facts, list current opt-in/module/test coverage,
  and keep baseline caveats tied to reproducible commands. Retain the existing
  editing, MSRV, environment-isolation, and verification rules.
  **Verify:** Cross-check against `Rakefile`, the manifests, workflow, and the
  baseline below; run `git diff --check`.

- [ ] **R2-20 — Correct the historical sysexits attribution path.**
  **Evidence (documentation review):** `CREDITS.md` describes
  `lib/clientele/src/sysexits.rs` and `clientele::sysexits`, neither of which exists.
  Since 0.3.4, the API is re-exported from `known_errors::sysexits` in
  `lib/clientele/src/lib.rs`.
  **Acceptance:** Explain the historical attribution and current dependency/API
  location accurately while retaining the existing BSD notice and provenance.
  **Verify:** Check the public re-exports and changelog against the revised text;
  run `git diff --check`.

## P3 — Targeted API and performance extensions

- [ ] **R2-21 — Expose reusable typed sort parsing outside a Clap invocation.**
  **Evidence (API opportunity):** In `lib/clientele/src/options/sort.rs`,
  `parse_sort_keys` already accepts a key parser, but it is private. Only
  `SortKeys<String>` implements `FromStr`; typed consumers of config/environment
  values must duplicate the comma/direction grammar or construct a Clap invocation.
  **Acceptance:** Add an explicit typed parsing constructor accepting a key
  parser, reusing the existing grammar and first-error behavior. Preserve string
  `FromStr`, typed `ValueEnum` parsing, whitespace handling, and default semantics.
  Document an example with a typed key that does not implement `ValueEnum`.
  **Verify:** `cargo test -p clientele --no-default-features --features clap --locked`,
  including mixed directions, invalid components, and custom key-parser errors.

- [ ] **R2-22 — Reuse a per-call search context during Windows collection.**
  **Evidence (source-level performance opportunity):** Windows `collect` scans
  `PATH`, then calls `resolve_command` for every unique name. Each call rereads and
  reparses `PATH` and `PATHEXT`; candidates also incur `exists` before a second
  metadata lookup in `filter_file`. No performance regression is asserted without
  measurement.
  **Acceptance:** Measure a synthetic multi-directory/multi-extension search,
  then reuse parsed search state within one collection call and remove proven
  redundant filesystem work. Preserve a fresh environment snapshot for subsequent
  public calls, OS-path spelling, filtering, and precedence. Resolve R2-03 through
  R2-05 first so optimization preserves the corrected semantics.
  **Verify:** Record before/after measurements and run native Windows discovery
  regressions, including repeated `PATH` directories and extension collisions.

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
`clap,tracing`; `clap,color,tracing`; `error-stack`; `std,error-stack`;
`gofer`; `std,gofer`.
Preserve optional-example/test gates in `lib/clientele/Cargo.toml`.

For ANSI stripping and its doctests without optional features:

```sh
cargo test -p clientele --no-default-features --locked
```

For argument-file recursion, aliases, repeated includes, and disabled expansion:

```sh
cargo test -p clientele --test args_os --locked
cargo test -p clientele --test args_os --no-default-features --features std,argfile --locked
cargo test -p clientele --test args_os --no-default-features --features std --locked
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

For optional generators, test each feature independently and together:

```sh
cargo test -p clientele --no-default-features --features completions --locked
cargo test -p clientele --no-default-features --features manpages --locked
cargo check -p clientele --all-targets --no-default-features --features completions,manpages --locked
```

For version-bump tooling, `rake test` runs isolated Cargo workspaces and verifies
that historical versions are preserved. CI quality gates also require:

```sh
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked
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

### Observed baseline on 2026-10-03

Host: aarch64 macOS; stable Rust/Cargo 1.98.1, with Rust 1.97.0 also installed.

**Passed in this review and follow-up verification:**

- `cargo fmt --all -- --check`.
- `cargo test --workspace --locked` (42 library tests, integration targets, and
  19 doctests).
- `cargo check -p clientele --all-targets --no-default-features --locked`.
- Default and all-feature Clippy/rustdoc with the warning-denying commands above.
- `cargo +1.97.0 test --workspace --all-features --locked` (including both
  generators, error-stack, and 21 doctests).
- All-target isolated-feature checks for the public leaf features,
  plus key pairs/groups including `std,gofer`,
  `std,argfile,wild`, `std,dirs,camino`, `std,getenv,camino`, `std,tracing`,
  `clap,tracing`, `clap,color,tracing`, `std,error-stack`, and
  `completions,manpages`.
- Standalone `gofer` tests on stable and Rust 1.97.0, plus
  `cargo +1.97.0 check -p clientele --lib --no-default-features --features gofer --locked`.
- `cargo package -p clientele --locked --target-dir target` and packaged
  default/no-default-feature doctests (19 and 4 doctests respectively).
- `rake test` (2 tests, 30 assertions) and
  `cargo run --locked --example skeleton -- config`.
- `cargo check -p clientele --all-targets --all-features --locked --target <target>`
  for `aarch64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`.

**Failures and limitations:**

- The downstream derive snippet failed until the `clap` module import was added;
  case-insensitive lookup, failed-bump, and closed-pipe probes exposed the
  behaviors described in R2-05 through R2-09.
- Ordinary quality checks are warning-free with current allowances. A diagnostic
  `cargo rustc -p clientele --lib --all-features --locked -- --force-warn unused`
  reports five suppressed warnings; see R2-17.
- No native Linux/Windows runtime tests were executed during this review. In
  particular, the Windows findings in R2-03/R2-04 are not claimed as native
  reproductions. Recheck and report failures rather than suppressing them.

For maintenance of this backlog alone, review file references and run
`git diff --check`; code/test changes require the applicable checks above.
