# Clientele.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.97%2B-blue)](https://endoflife.date/rust)
[![Package on Crates.io](https://img.shields.io/crates/v/clientele)](https://crates.io/crates/clientele)
[![Documentation](https://img.shields.io/docsrs/clientele?label=docs.rs)](https://docs.rs/clientele)

**Clientele** makes it easy to write superb command-line utilities in Rust
that follow consistent best practices on all target platforms including Linux,
macOS, and Windows. It packages and re-exports [clap], [camino],
[dotenvy], [wild], [argfile], and [getenv] into a single easy dependency.

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

## ✨ Features

- Reusable [clap] flags for color, debugging, verbosity, license, and version.
- Loads environment variables from `.env` files (using the [dotenvy] crate).
- Provides convenience getters for common variables (using the [getenv] crate).
- Expands wildcard arguments (globs) on Windows (using the [wild] crate).
- Expands @argfiles similarly to [`javac`] or Python (using the [argfile] crate).
- Provides the [`Utf8Path`] and [`Utf8PathBuf`] types (using the [camino] crate).
- Recommends use of the [`sysexits.h(3)`] exit codes (see [known-errors]).
- Dependency re-exports and error conversions to `sysexits.h` exit codes.
- External subcommand discovery with sorted, deduplicated `PATH` listings.
- Stream-aware color detection, styled help, and ANSI/OSC hyperlink stripping.
- Tracing formats and fallible subscriber initialization, honoring CLI options.
- Opt-in shell completion and man-page generation from [clap] definitions.
- Showcases how to structure a CLI program in Rust (see the [examples](#-examples)).
- Supports opting out of any feature using comprehensive [feature flags].

## 🛠️ Prerequisites

- [Rust] 1.97+ (Rust 2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add clientele
```

<details>
<summary>Configuration in <code>Cargo.toml</code></summary>

Default feature bundle:

```toml
[dependencies]
clientele = "0.5"
```

Focused CLI dependency:

```toml
[dependencies]
clientele = { version = "0.5", default-features = false, features = ["clap", "dotenv", "argfile", "wild"] }
```

- Add `color` for colored help; `tracing` for logging setup.
- Add `completions` or `manpages` for generators.
- Cargo unifies features across dependencies.

</details>

## 👉 Examples

### Parsing Arguments

```rust,no_run
# #[cfg(all(feature = "clap", feature = "dotenv", feature = "argfile", feature = "wild"))]
# {
use clientele::crates::clap::{self, Parser};

#[derive(Parser)]
struct Options {
    #[arg(long)]
    name: Option<String>,
}

clientele::dotenv().ok();
let options = Options::parse_from(clientele::args_os()?);
# }
# Ok::<(), clientele::SysexitsError>(())
```

### Running the Example

The [skeleton] demonstrates standard flags, color, tracing, sysexits, and
broken-pipe handling:

```bash
cargo run --example skeleton -- --help
cargo run --example skeleton -- -vvv config
```

## 📚 Reference

[docs.rs/clientele](https://docs.rs/clientele)

### Standard Options

Flatten [`StandardOptions`] into a Clap parser; applications handle the recorded flags.

| Flag | Default | Purpose |
| --- | --- | --- |
| `--color <auto\|always\|never>` | `auto` | Output color policy (requires `color`) |
| `-d`, `--debug` | Off | Debug output; tracing level TRACE |
| `-v`, `--verbose...` | `0` | Tracing levels ERROR → WARN → INFO → DEBUG |
| `--license` | Off | Request license information |
| `-V`, `--version` | Off | Request version information |

### Feature Flags

| Feature(s) | Effect |
| --- | --- |
| `default` | `all` + `std` |
| `all` | All functionality below except the opt-ins; not Cargo's `--all-features` |
| `std` | Argument, path, and exit helpers; dependency standard-library support |
| `clap` | Standard options and sort keys; enables `std` |
| `argfile`, `wild` | Enhance `args_os()`; requires `std` |
| `dotenv`, `getenv` | `.env` loading; environment getters (`std,getenv`) |
| `dirs`, `camino` | Native/UTF-8 paths; XDG helpers need `std,getenv,camino`; see [`paths::*`] |
| `subcommands` | Executable discovery; requires `std` |
| `color`, `unicode` | Color for enabled Clap/tracing dependencies; Unicode for enabled Clap |
| `tracing` | Formats with `std`; options-based initialization also needs `clap` |
| `parse` | `parse-byteunit` + `parse-duration` + reserved, empty `parse-datetime` |
| `serde`, `serde-json` | Camino serialization; JSON re-export and error conversion |
| `gofer`, `tokio` | Fetching/runtime re-exports and error conversions; `gofer` enables `std` |
| **Opt-in:** `completions` | Bash, Elvish, Fish, PowerShell, Zsh generators; enables `clap,std` |
| **Opt-in:** `manpages` | ROFF manual-page generator; enables `clap,std` |
| **Opt-in:** `error-stack` | `SysexitsError` report contexts, with or without `std` |
| **Opt-in:** `unstable` | Forward Dogma's unstable feature |

### Interoperability

Enabled dependencies are re-exported through [`clientele::crates`]. Versions below are manifest requirements.

| Feature | Crate(s) | Version | Integration |
| --- | --- | --- | --- |
| Always | [dogma] | 0.3.0 | Shared vocabulary; defaults disabled |
| Always | [known-errors] | 0.1.2 | `SysexitsError`, `SysexitsResult`; `abort!`, `exit` with `std` |
| `clap` | [clap] | 4.5 | Derive parsing, `StandardOptions`, typed sort keys |
| `camino` | [camino] | 1.1 | `Utf8Path`, `Utf8PathBuf` and path helpers |
| `dirs` | [dirs] | 6.0 | Native home-directory resolution |
| `dotenv` | [dotenvy] | 0.15 | `dotenv()` |
| `getenv` | [getenv] | 0.1 | `envs` re-export and XDG paths |
| `argfile`, `wild` | [argfile], [wild] | 0.2, 2 | Argument-file and Windows glob expansion |
| `tracing` | [tracing-core], [tracing-subscriber] | 0.1, 0.3.19 | Level conversions, global/scoped subscribers, reusable formats |
| `serde` | [serde] | 1 | Re-export; serialization for already-enabled Camino |
| `serde-json` | [serde_json] | 1 | Enables Serde; JSON-error → sysexits conversion |
| `gofer` | [gofer] | 0.1.8 | Fetch-error → sysexits conversion |
| `tokio` | [tokio] | 1 | Runtime; join-error → sysexits conversion |
| `error-stack` | [error-stack] | 0.5 | Sysexits report contexts |
| `parse-byteunit`, `parse-duration` | [ubyte], [duration-str] | 0.10, 0.15 | Parser re-exports |
| `completions`, `manpages` | [clap_complete], [clap_mangen] | 4.6, 0.3 | Clap-based generators |

- `known-errors` exposes the listed APIs, rather than a crate re-export.
- `serde` alone does not enable JSON; `camino` remains optional.
- Add a direct `tracing` dependency to emit events; subscriber setup is explicit.

## 👨‍💻 Development

```bash
git clone https://github.com/dryrust/clientele.rs.git
```

See [AGENTS.md](https://github.com/dryrust/clientele.rs/blob/master/AGENTS.md)
for contributor guidance and
[TODO.md](https://github.com/dryrust/clientele.rs/blob/master/TODO.md) for
review status and the enhancement backlog.

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/dryrust/clientele.rs&text=Clientele.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dryrust/clientele.rs&title=Clientele.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dryrust/clientele.rs&t=Clientele.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dryrust/clientele.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/dryrust/clientele.rs)

[Rust]: https://rust-lang.org
[changelog]: https://github.com/dryrust/clientele.rs/blob/master/CHANGES.md
[feature flags]: https://docs.rs/crate/clientele/latest/features
[skeleton]: https://github.com/dryrust/clientele.rs/blob/master/lib/clientele/examples/skeleton/main.rs

[argfile]: https://crates.io/crates/argfile
[camino]: https://crates.io/crates/camino
[clap]: https://crates.io/crates/clap
[clap_complete]: https://crates.io/crates/clap_complete
[clap_mangen]: https://crates.io/crates/clap_mangen
[dirs]: https://crates.io/crates/dirs
[dogma]: https://crates.io/crates/dogma
[dotenvy]: https://crates.io/crates/dotenvy
[duration-str]: https://crates.io/crates/duration-str
[error-stack]: https://crates.io/crates/error-stack
[getenv]: https://crates.io/crates/getenv
[gofer]: https://crates.io/crates/gofer
[known-errors]: https://crates.io/crates/known-errors
[serde]: https://crates.io/crates/serde
[serde_json]: https://crates.io/crates/serde_json
[tokio]: https://crates.io/crates/tokio
[tracing-core]: https://crates.io/crates/tracing-core
[tracing-subscriber]: https://crates.io/crates/tracing-subscriber
[ubyte]: https://crates.io/crates/ubyte
[wild]: https://crates.io/crates/wild

[`StandardOptions`]: https://docs.rs/clientele/latest/clientele/struct.StandardOptions.html
[`SysexitsError`]: https://docs.rs/clientele/latest/clientele/enum.SysexitsError.html
[`Utf8Path`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html
[`Utf8PathBuf`]: https://docs.rs/camino/latest/camino/struct.Utf8PathBuf.html
[`args_os()`]: https://docs.rs/clientele/latest/clientele/fn.args_os.html
[`dotenv()`]: https://docs.rs/clientele/latest/clientele/fn.dotenv.html
[`envs::*`]: https://docs.rs/getenv/latest/getenv/index.html
[`paths::*`]: https://docs.rs/clientele/latest/clientele/paths/index.html
[`clientele::crates`]: https://docs.rs/clientele/latest/clientele/crates/index.html
