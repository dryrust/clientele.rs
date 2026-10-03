# Clientele.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.97%2B-blue)](https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/)
[![Package](https://img.shields.io/crates/v/clientele)](https://crates.io/crates/clientele)
[![Documentation](https://docs.rs/clientele/badge.svg)](https://docs.rs/clientele/)

**Clientele** makes it easy to write superb command-line utilities in Rust
that follow consistent best practices on all target platforms including Linux,
macOS, and Windows. It packages and re-exports [clap], [camino],
[dotenvy], [wild], [argfile], and [getenv] into a single easy
dependency.

## ✨ Features

- Showcases how to structure a CLI program in Rust (see the [examples](#-examples)).
- Loads environment variables from `.env` files (using the [dotenvy] crate).
- Provides convenience getters for common variables (using the [getenv] crate).
- Expands wildcard arguments (globs) on Windows (using the [wild] crate).
- Expands @argfiles similarly to [`javac`] or Python (using the [argfile] crate).
- Defines a standard set of essential CLI options (using the [clap] crate).
- Provides the [`Utf8Path`] and [`Utf8PathBuf`] types (using the [camino] crate).
- Recommends use of the [`sysexits.h(3)`] exit codes (see [known-errors]).
- Supports opting out of any feature using comprehensive feature flags.
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.97+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add clientele
```

### Installation in `Cargo.toml` (with default features)

```toml
[dependencies]
clientele = "0.4"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
clientele = { version = "0.4", default-features = false, features = ["dotenv"] }
```

### Focused CLI dependency

For argument parsing, `.env` loading, @argfiles, and Windows wildcard expansion:

```toml
[dependencies]
clientele = { version = "0.4", default-features = false, features = ["clap", "dotenv", "argfile", "wild"] }
```

`clap` supplies `std`. Load `.env` before expanding arguments, then pass the
resulting OS strings to Clap:

```no_run
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

Importing `clap` itself with `self` lets the derive macro resolve its generated
paths without a separate direct Clap dependency.

This selection avoids Clientele's optional runtime, serialization, and byte-unit
or duration parser dependencies. Add `color` for colored Clap output. Add `tracing`
for logging initialization through `clientele::tracing` and `StandardOptions`;
add `color` as well for colored logs. Logging initialization is explicit, and
applications emitting events can depend directly on the `tracing` crate.
Cargo unifies dependency features, so another dependency may enable additional
Clientele features in the same build.

### Feature selection

- Defaults enable `all` and `std`. The `all` feature is a curated bundle;
  it excludes `error-stack`, `unstable`, `completions`, and `manpages`.
- Cargo's `--all-features` enables every feature, including those opt-ins.
- `completions` enables shell completion generation and implies `clap,std`;
  see the [completion guide](https://docs.rs/clientele/latest/clientele/completions/).
- `manpages` independently enables ROFF manual-page generation and implies
  `clap,std`; see the [man-page guide](https://docs.rs/clientele/latest/clientele/manpages/).
- `clap` enables `std`. Argument expansion and subcommand discovery require
  `std`; discovery additionally requires `subcommands`.
- Native temporary paths require `std`; native home paths also require `dirs`.
  UTF-8 variants require `camino`. Environment-only and XDG paths require
  `std,getenv,camino`.
- Tracing formats require `std,tracing`; subscriber initialization also requires
  `clap`. `color` enables ANSI support only in already-enabled Clap/tracing
  dependencies. `unicode` similarly augments an already-enabled Clap.
- `serde-json` enables `serde`. `parse` groups byte-unit and duration parsers
  plus the reserved, currently empty `parse-datetime` feature.
- Disabling defaults does not provide `no_std` support. `error-stack` integration
  works with or without the `std` feature.

Enabled dependencies are available through [`clientele::crates`], for example
`clientele::crates::clap::Parser` with `clap`. Use this supported entry point to
share Clientele's dependency versions. See the [crate rustdoc] for API-specific
feature requirements.

## 👉 Examples

See [`examples/skeleton/main.rs`] for a complete example.

### Importing the Library

```rust
use clientele::*;
```

### Running the Example

```bash
cargo run --example skeleton
```

## 📚 Reference

### Options

#### [`StandardOptions`]

```text
Options:
      --color <COLOR>  Set the color output mode [default: auto] [possible values: auto, always, never]
  -d, --debug          Enable debugging output
      --license        Show license information
  -v, --verbose...     Enable verbose output (may be repeated for more verbosity)
  -V, --version        Print version information
  -h, --help           Print help
```

### Integrations

Crate (Feature) | Version | Usage | Summary
:--- | :--- | :--- | :---
[argfile] &nbsp;<sub>(`"argfile"`)</sub> | 0.2 | [![argfile](https://docs.rs/argfile/badge.svg)](https://docs.rs/argfile/) | Enhances [`args_os()`] to expand @argfiles
[camino] &nbsp;<sub>(`"camino"`)</sub> | 1.1 | [![camino](https://docs.rs/camino/badge.svg)](https://docs.rs/camino/) | UTF-8 path types and UTF-8 [`paths::*`] helpers
[clap] &nbsp;<sub>(`"clap"`)</sub> | 4.5 | [![clap](https://docs.rs/clap/badge.svg)](https://docs.rs/clap/) | Provides [`StandardOptions`]
[dirs] &nbsp;<sub>(`"dirs"`)</sub> | 6.0 | [docs](https://docs.rs/dirs/) | Native home-directory resolution with `std`
[dotenvy] &nbsp;<sub>(`"dotenv"`)</sub> | 0.15 | [![dotenvy](https://docs.rs/dotenvy/badge.svg)](https://docs.rs/dotenvy/) | Provides [`dotenv()`]
[duration-str] &nbsp;<sub>(`"parse-duration"`)</sub> | 0.15 | [docs](https://docs.rs/duration-str/) | Duration parser re-export
[error-stack] &nbsp;<sub>(`"error-stack"`)</sub> | 0.5 | [docs](https://docs.rs/error-stack/) | [`SysexitsError`] report contexts
[getenv] &nbsp;<sub>(`"getenv"`)</sub> | 0.1 | [![getenv](https://docs.rs/getenv/badge.svg)](https://docs.rs/getenv/) | [`envs::*`] with `std`; environment paths also need `camino`
[gofer] &nbsp;<sub>(`"gofer"`)</sub> | 0.1 | [docs](https://docs.rs/gofer/) | Fetching and known-error integration
[serde] &nbsp;<sub>(`"serde"`)</sub> | 1 | [docs](https://docs.rs/serde/) | Serialization integration and re-export
[serde_json] &nbsp;<sub>(`"serde-json"`)</sub> | 1 | [docs](https://docs.rs/serde_json/) | JSON support; enables `serde`
[tokio] &nbsp;<sub>(`"tokio"`)</sub> | 1 | [docs](https://docs.rs/tokio/) | Runtime re-export and known-error integration
[tracing-core] &nbsp;<sub>(`"tracing"`)</sub> | 0.1 | [![tracing-core](https://docs.rs/tracing-core/badge.svg)](https://docs.rs/tracing-core/) | Converts [`StandardOptions`] to `tracing_core::LevelFilter`
[tracing-subscriber] &nbsp;<sub>(`"tracing"`)</sub> | 0.3 | [docs](https://docs.rs/tracing-subscriber/) | Formats with `std`; initializer also needs `clap`
[ubyte] &nbsp;<sub>(`"parse-byteunit"`)</sub> | 0.10 | [docs](https://docs.rs/ubyte/) | Byte-unit parser re-export
[wild] &nbsp;<sub>(`"wild"`)</sub> | 2 | [![wild](https://docs.rs/wild/badge.svg)](https://docs.rs/wild/) | Enhances [`args_os()`] to support globs on Windows
<img width="220" height="1"/> | <img width="110" height="1"/> | <img width="100" height="1"/> | &nbsp;

## 👨‍💻 Development

```bash
git clone https://github.com/dryrust/clientele.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/dryrust/clientele.rs&text=Clientele.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dryrust/clientele.rs&title=Clientele.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dryrust/clientele.rs&t=Clientele.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dryrust/clientele.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/dryrust/clientele.rs)

[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html
[`examples/skeleton/main.rs`]: lib/clientele/examples/skeleton/main.rs

[`javac`]: https://docs.oracle.com/javase/7/docs/technotes/tools/windows/javac.html#commandlineargfile
[`sysexits.h(3)`]: https://man7.org/linux/man-pages/man3/sysexits.h.3head.html

[argfile]: https://crates.io/crates/argfile
[camino]: https://crates.io/crates/camino
[clap]: https://crates.io/crates/clap
[dirs]: https://crates.io/crates/dirs
[duration-str]: https://crates.io/crates/duration-str
[dotenvy]: https://crates.io/crates/dotenvy
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
[crate rustdoc]: https://docs.rs/clientele/latest/clientele/
