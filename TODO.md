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

**Status:** 12 outstanding leaf tasks: 3 P1, 7 P2, and 2 P3. Evidence below
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
| Feature gates, dependency re-exports, consumer builds | Serde isolation and scoped tracing consumer checks pass; broader feature coverage in R2-14 |
| Arguments and skeleton CLI | Argument-file contracts pass; native Windows wildcard coverage remains in R2-13 |
| Executable discovery | Case-sensitive-prefix mismatch reproduced on macOS; Windows extension/identity findings and OS-path coverage gaps; R2-03 through R2-05, R2-16, R2-22 |
| Color scanning, ANSI/OSC stripping, sort parsing and checked SQL | Existing implementations and regression tests reviewed; optional typed-parser reuse in R2-21 |
| Native/UTF-8/XDG paths and tracing | Existing path, format, color, and global-initialization tests pass locally; isolated-feature coverage in R2-14 |
| Completions, manpages, error-stack, packaging | All-feature MSRV tests and packaged default/minimal doctests pass; isolated-feature and quality gates in R2-14, R2-18 |
| CI, Rake, Make, project documentation | Missing Ruby CI coverage and stale guidance/credits; R2-15, R2-19, R2-20 |

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

## P2 — Regression coverage and automation

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
  **Evidence (coverage gap):** `rake test` currently passes its three tests and
  52 assertions, but no workflow step runs `tests/version_bump_test.rb`. Rust tests
  and packaging checks cannot detect regressions in `Rakefile`.
  **Acceptance:** Set up reproducible Ruby, Rake, and Minitest dependencies in one
  CI job and execute `rake test`. Keep fixtures isolated/offline and surface failures
  as job failures, including the existing failed-update rollback/retry regression.
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

For Serde/JSON isolation and integration, inspect the Serde-only normal dependency
tree and run `serde_integration` with `serde,camino`, `serde-json`, and
`std,serde-json` under `--no-default-features --locked`.

For ANSI stripping and its doctests without optional features:

```sh
cargo test -p clientele --no-default-features --locked
```

For argument-file recursion, aliases, line parsing, empty files, atomic errors,
expansion after `--`, Unix argv[0] overrides, and disabled-feature pass-through:

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

For skeleton color, argument-file diagnostics, closed pipes, and write errors,
run the existing CLI driver:

```sh
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,dotenv --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,color,dotenv --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,dotenv,argfile --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,dotenv,tracing --locked
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

For derive examples and the skeleton in a consumer without a direct Clap dependency:

```sh
cargo test --manifest-path tests/consumer/Cargo.toml --locked --target-dir target
cargo test --manifest-path tests/consumer/Cargo.toml --all-features --locked --target-dir target
cargo run --manifest-path tests/consumer/Cargo.toml --no-default-features --features tracing --example scoped_tracing --locked --target-dir target
```

For optional generators, test each feature independently and together:

```sh
cargo test -p clientele --no-default-features --features completions --locked
cargo test -p clientele --no-default-features --features manpages --locked
cargo check -p clientele --all-targets --no-default-features --features completions,manpages --locked
```

For version-bump tooling, `rake test` runs isolated Cargo workspaces and verifies
historical versions, rollback of partial updates, and retry behavior with present
and absent lockfiles. CI quality gates also require:

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
- `cargo rustc -p clientele --lib --all-features --locked -- --force-warn unused`
  is warning-free after removing blanket suppression and unused private scaffolding.
- `cargo +1.97.0 test --workspace --all-features --locked` (including both
  generators, error-stack, and 21 doctests).
- All-target isolated-feature checks for the public leaf features,
  plus key pairs/groups including `std,gofer`,
  `std,argfile,wild`, `std,dirs,camino`, `std,getenv,camino`, `std,tracing`,
  `clap,tracing`, `clap,color,tracing`, `std,error-stack`, and
  `completions,manpages`.
- Standalone `gofer` tests on stable and Rust 1.97.0, plus
  `cargo +1.97.0 check -p clientele --lib --no-default-features --features gofer --locked`.
- Downstream consumer tests above (6 minimal and 9 color/tracing doctests),
  including all-feature consumer tests on Rust 1.97.0.
- `cargo package -p clientele --locked --target-dir target` and packaged
  default/no-default-feature doctests (19 and 4 doctests respectively).
- `rake test` (3 tests, 52 assertions) and
  `cargo run --locked --example skeleton -- config`.
- `cargo check -p clientele --all-targets --all-features --locked --target <target>`
  for `aarch64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`.

**Failures and limitations:**

- The case-insensitive lookup mismatch remains as described in R2-05.
- No native Linux/Windows runtime tests were executed during this review. In
  particular, the Windows findings in R2-03/R2-04 are not claimed as native
  reproductions. Recheck and report failures rather than suppressing them.

For maintenance of this backlog alone, review file references and run
`git diff --check`; code/test changes require the applicable checks above.
