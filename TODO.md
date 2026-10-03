# Clientele enhancement backlog

Note: the next release is going to be 0.5.0, meaning backwards
incompatibility does not need to be strictly preserved.

Review snapshot: 2026-10-03. This file records project review history and
verification evidence, with enough context to continue future reviews.
Task IDs are stable and may contain gaps. Recheck the relevant code before
implementing a task. The `R2-*` IDs identify findings from this review; completed
items from the previous review have not been reopened.

## Working method

- Follow [AGENTS.md](AGENTS.md). This is a Rust 2024 library workspace with MSRV
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

**Status:** No outstanding leaf tasks. All findings from this review have been
implemented and verified. The dated baseline and follow-up sections below are
historical snapshots; the later native verification records supersede their
pending-platform notes. Do not treat cross-compilation as native verification.

Priority: **P1** correctness/reliability, **P2** API/UX/maintenance, **P3** optional
polish or feature growth. Priorities do not override an explicitly selected task.

## Review coverage

Reviewed all tracked source modules, integration/unit tests, the skeleton example,
manifests and lockfile, documentation, and CI/release/developer tooling. Original
behavioral probes ran on macOS; subsequent native Windows/Linux/macOS CI evidence
is recorded below. Cross-compilation establishes build coverage only.

| Area | Review outcome |
| --- | --- |
| Feature gates, dependency re-exports, consumer builds | Standalone-feature builds, weak-feature guards, Serde isolation, and scoped tracing checks pass |
| Arguments and skeleton CLI | Argument-file contracts and R2-13 native Windows raw wildcard/argfile ordering tests pass |
| Executable discovery | R2-03/R2-04 verified on Windows; R2-05 on Windows/macOS/Linux; R2-16 on Windows/Linux; R2-22 per-call search reuse measured and verified on Windows |
| Color scanning, ANSI/OSC stripping, sort parsing and checked SQL | Existing regressions and the typed `parse_with` callback contract pass |
| Native/UTF-8/XDG paths and tracing | Path, format, color, global-initialization, and focused feature-combination tests pass locally |
| Completions, manpages, error-stack, packaging | Isolated-feature tests, all-feature quality gates, and packaged default/minimal doctests pass |
| CI, Rake, Make, project documentation | Locked Ruby CI coverage, current contributor guidance, and historical attribution are in place |

## Verification and baseline

Run relevant checks from the repository root as required by `AGENTS.md`:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo check -p clientele --all-targets --no-default-features --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked
```

Add focused tests for the selected leaf. For feature changes, check affected
combinations with `cargo check -p clientele --all-targets --no-default-features
--features <set> --locked`, replacing `<set>` with the applicable feature list.
Useful sets: `clap`; `std,subcommands`; `std,getenv,camino`; `std,tracing`;
`clap,tracing`; `clap,color,tracing`; `error-stack`; `std,error-stack`;
`gofer`; `std,gofer`.
Preserve optional-example/test gates in `lib/clientele/Cargo.toml`.

CI's manifest-driven feature driver checks library-only and all-target builds for
every declared feature, then runs focused tests and weak-feature dependency guards:

```sh
python3 tests/check_features.py
python3 tests/check_features.py --toolchain 1.97.0
```

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

For raw Windows wildcard quoting and wildcard-before-argfile ordering (native
Windows only; non-Windows targets report that the scenarios require Windows):

```sh
cargo test -p clientele --test args_wild --no-default-features --features std,wild --locked
cargo test -p clientele --test args_wild --no-default-features --features std,wild,argfile --locked
cargo test -p clientele --test args_wild --locked
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

For version-bump tooling, install the locked development gems and run the isolated
Cargo fixtures, including rollback of partial updates and absent lockfiles:

```sh
bundle install
bundle exec rake test
```

CI uses Ruby 4.0.6 and `Gemfile.lock`. Frozen bundle installation and all 52 Ruby
assertions pass locally. Rust quality gates also require:

```sh
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps --locked
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-default-features --no-deps --locked
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-default-features --features clap --no-deps --locked
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
- `cargo test --workspace --locked` (45 library tests, integration targets, and
  21 doctests).
- `cargo check -p clientele --all-targets --no-default-features --locked`.
- Default and all-feature Clippy/rustdoc with the warning-denying commands above.
- `cargo rustc -p clientele --lib --all-features --locked -- --force-warn unused`
  is warning-free after removing blanket suppression and unused private scaffolding.
- `cargo +1.97.0 test --workspace --all-features --locked` (including both
  generators, error-stack, and 23 doctests).
- The manifest-driven feature checks passed locally on stable and Rust 1.97.0.
- All-target isolated-feature checks for the public leaf features,
  plus key pairs/groups including `std,gofer`,
  `std,argfile,wild`, `std,dirs,camino`, `std,getenv,camino`, `std,tracing`,
  `clap,tracing`, `clap,color,tracing`, `std,error-stack`, and
  `completions,manpages`.
- Standalone `gofer` tests on stable and Rust 1.97.0, plus
  `cargo +1.97.0 check -p clientele --lib --no-default-features --features gofer --locked`.
- Downstream consumer tests above (7 minimal and 10 color/tracing doctests),
  including all-feature consumer tests on Rust 1.97.0.
- `cargo package -p clientele --locked --target-dir target` and packaged
  default/no-default-feature doctests (21 and 4 doctests respectively).
- `bundle exec rake test` (3 tests, 52 assertions) and
  `cargo run --locked --example skeleton -- config`.
- `cargo check -p clientele --all-targets --all-features --locked --target <target>`
  for `aarch64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`.

**Failures and limitations:**

- The R2-05 case-insensitive lookup mismatch is fixed and its regressions pass on
  macOS; native Windows and case-sensitive control runs remain outstanding.
- No native Linux/Windows runtime tests were executed during this review. In
  particular, the Windows findings in R2-03/R2-04 are not claimed as native
  reproductions. Recheck and report failures rather than suppressing them.

For maintenance of this backlog alone, review file references and run
`git diff --check`; code/test changes require the applicable checks above.

### Follow-up verification on 2026-10-03

Ten atomic implementation/test steps advanced R2-03, R2-04, R2-05, R2-13, and
R2-16. Their progress notes retain the outstanding native verification rather
than marking cross-compiled tests as executed. R2-22 still needs measurement and
implementation after that verification.

- Passed formatting, default workspace tests (46 library tests and 21 doctests),
  no-default-feature all-target checks, and default/all-feature Clippy and rustdoc
  with warnings denied. A test-clone Clippy finding was corrected before completion.
- Passed Rust 1.97.0 all-feature workspace tests (including 23 doctests).
- Passed minimal `std,subcommands` discovery tests on case-insensitive macOS.
  Non-Unicode filesystem fixtures reported a skip because creation returned EILSEQ.
- Windows all-feature all-target checks and minimal discovery/wild/argfile checks
  passed, including Windows-target Clippy with `std,wild,argfile,subcommands`.
  Linux all-target checks with those features passed as well.
- Raw wildcard scenarios require native Windows; the macOS driver reports this
  explicitly. Native Windows and supporting Unix-filesystem runs remain pending.

### R2-03 native verification on 2026-10-03

Closed R2-03 after reviewing native Windows CI for commit
`2df1ea64ef0e06757bd6140af34beb75df1985f6` in
[run 37141180579](https://github.com/dryrust/clientele.rs/actions/runs/37141180579).
Both [Rust 1.97.0](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770523)
and [stable](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770537)
Windows jobs passed default, minimal `std,subcommands`, and all-feature tests.

- The parser's separator regression passed; the minimal feature suite passed all
  17 library tests, including precedence, Unicode, and non-trimming coverage.
- The `subcommands_path` driver exercises `.bad/name`, `.bad\name`, and their
  combination, both alone and before `.BAT`, through `find` and `collect`; it also
  checks explicit-file fallback. Its child failures propagate to the CI job.
- `tests/check_features.py` runs the minimal suite with `--no-default-features
  --features std,subcommands --locked`; both Windows logs confirm this combination
  completed successfully. This supplies the native execution missing from the
  earlier local verification. Other open items require their own evidence review.

### R2-04 native verification on 2026-10-03

Closed R2-04 after reviewing the implementation, collision fixtures, and native
Windows logs for commit `2df1ea64ef0e06757bd6140af34beb75df1985f6` in
[run 37141180579](https://github.com/dryrust/clientele.rs/actions/runs/37141180579).
The current source and tests match that commit; the intervening change only
recorded R2-03 verification in this file.

- Both [Rust 1.97.0](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770523)
  and [stable](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770537)
  executed `subcommands_find`, `subcommands_list`, and `subcommands_order`
  successfully with defaults and all features. Their feature-driver logs also
  confirm the minimal `std,subcommands` suite completed successfully.
- Shared fixtures cover the same-directory `report.v1`/`report.v1.bat` collision.
  The order driver covers reversed/repeated `PATH` directories, `.BAT`/`.CMD`
  precedence, `.V1` as a recognized extension, `task.bat`/`task.bat.bat` ambiguity,
  explicit-file fallback, sorted/deduplicated listings, and name/path round trips.
  Child failures propagate to the parent test and CI job.
- The compatibility decision is documented in `SubcommandsProvider::find` and
  `CHANGES.md`: search logical stems across all of `PATH` before exact-filename
  fallback, so `report.v1.bat` wins over `report.v1` even in a later directory.

### R2-05 native verification on 2026-10-03

Closed R2-05 using [CI run 37141180579](https://github.com/dryrust/clientele.rs/actions/runs/37141180579)
at `2df1ea64ef0e06757bd6140af34beb75df1985f6`; source and tests are unchanged.
Reviewed successful stable jobs for
[Windows](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770537),
[macOS](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770602),
and [Linux](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770629).
The case-collision driver reported case-insensitive filesystems on Windows/macOS
and a case-sensitive filesystem on Linux. Default/all-feature suites passed on
all three; minimal `std,subcommands` suites also passed on Windows/Linux.
Fixtures verify literal prefixes and logical names, uppercase Windows extensions,
reversed/repeated `PATH` precedence, and exact name/path round trips.

### R2-13 native verification on 2026-10-03

Closed R2-13 after reviewing `args_wild` and the successful
[stable Windows job](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770537)
at `2df1ea64ef0e06757bd6140af34beb75df1985f6`. The default/all-feature logs show
the native executable running; the feature driver successfully completed both
`std,wild` and `std,argfile,wild` without defaults. The driver uses `raw_arg`,
checks quoted/unquoted and unmatched patterns, spaces in matching filenames,
wildcards before argfile loading, and literal direct/nested argfile patterns.
All child failures propagate; these Windows branches have no skip path.

### R2-16 native verification on 2026-10-03

Closed R2-16 after reviewing `support/subcommands_native.rs` and the successful
[stable Windows](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770537)
and [stable Linux](https://github.com/dryrust/clientele.rs/actions/runs/37141180579/job/111255770629)
jobs at `2df1ea64ef0e06757bd6140af34beb75df1985f6`. Both ran `subcommands_path`
with defaults/all features and completed the minimal `std,subcommands` suite.
Neither log contains the fixture's unsupported-filesystem message, confirming
the native-path checks ran rather than skipped. Fixtures use invalid Unix bytes
or an unpaired Windows surrogate, compare exact `PathBuf` values through a
non-Unicode `PATH` parent, skip invalid executable names, and find valid neighbors.
Windows also checks explicit filename lookup. The macOS EILSEQ skip remains an
expected filesystem limitation, now covered by the native Linux execution.

### R2-22 measurements and native verification on 2026-10-03

Closed R2-22 after measuring the existing implementation, reusing parsed `PATH`
and `PATHEXT` plus lazy directory listings within each Windows call, and verifying
the optimized implementation natively. Directory-cache keys retain raw OS-path
spelling. The redundant `exists` probes had already been removed by R2-05;
this change removes repeated directory enumeration during collection/lookup.

Reproduce on Windows with:

```sh
cargo bench -p clientele --bench subcommands --no-default-features --features std,subcommands --locked
```

The unchanged release-mode fixture has six directories plus two repeated `PATH`
entries, three executable extensions, 97 logical commands, dotted exact-file
collisions, and nine timed collections after warmup. Expected listings and lookup
round trips are checked outside the timing. Stable Windows CI runs this benchmark
as a diagnostic, without a timing threshold.

| Revision | Median | Minimum | Maximum | Native Windows evidence |
| --- | --- | --- | --- | --- |
| `a9df518` baseline | 56.585 ms | 55.714 ms | 58.987 ms | [job 111264830458](https://github.com/dryrust/clientele.rs/actions/runs/37144251868/job/111264830458) |
| `e318b4d` optimized | 22.871 ms | 22.711 ms | 23.349 ms | [job 111266675407](https://github.com/dryrust/clientele.rs/actions/runs/37144888070/job/111266675407) |

The median is about 60% lower for this fixture. These are separate hosted-runner
measurements, not a universal performance guarantee.

- [CI run 37144888070](https://github.com/dryrust/clientele.rs/actions/runs/37144888070)
  passed all jobs: native stable/MSRV Windows, Linux, and macOS tests; focused
  feature suites; warning-denying quality gates; packaging; and release tooling.
- Windows default/minimal/all-feature discovery tests pass, including repeated
  directories, extension collisions, hidden/directory filtering, case matching,
  non-Unicode paths, and the new single-threaded freshness subprocess.
- The freshness regression changes `PATH` and `PATHEXT`, removes/creates files and
  directories, unsets required variables, and verifies that successive calls see
  new state while previously collected snapshots remain intact.
- Local default/minimal tests, formatting, Clippy, rustdoc, Windows-target Clippy,
  and Rust 1.97 Windows-target checks pass. The MSRV cross-check initially lacked
  its target installation; installing the target resolved that check failure.
