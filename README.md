# Clientele.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.81%2B-blue)](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0/)
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

- [Rust](https://rust-lang.org) 1.81+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add clientele
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
clientele = "0.3"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
clientele = { version = "0.3", default-features = false, features = ["dotenv"] }
```

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
[camino] &nbsp;<sub>(`"camino"`)</sub> | 1.1 | [![camino](https://docs.rs/camino/badge.svg)](https://docs.rs/camino/) | Prerequisite for [`paths::*`]
[clap] &nbsp;<sub>(`"clap"`)</sub> | 4.5 | [![clap](https://docs.rs/clap/badge.svg)](https://docs.rs/clap/) | Provides [`StandardOptions`]
[dotenvy] &nbsp;<sub>(`"dotenvy"`)</sub> | 0.15 | [![dotenvy](https://docs.rs/dotenvy/badge.svg)](https://docs.rs/dotenvy/) | Provides [`dotenv()`]
[getenv] &nbsp;<sub>(`"getenv"`)</sub> | 0.1 | [![getenv](https://docs.rs/getenv/badge.svg)](https://docs.rs/getenv/) | Provides [`envs::*`], prerequisite for [`paths::*`]
[tracing-core] &nbsp;<sub>(`"tracing"`)</sub> | 0.1 | [![tracing-core](https://docs.rs/tracing-core/badge.svg)](https://docs.rs/tracing-core/) | Implements `Into<tracing_core::Level>` for [`StandardOptions`]
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
[dirs]: #
[duration-str]: #
[dotenvy]: https://crates.io/crates/dotenvy
[error-stack]: #
[getenv]: https://crates.io/crates/getenv
[gofer]: #
[known-errors]: https://crates.io/crates/known-errors
[serde]: #
[serde_json]: #
[tokio]: #
[tracing-core]: https://crates.io/crates/tracing-core
[ubyte]: #
[wild]: https://crates.io/crates/wild

[`StandardOptions`]: https://docs.rs/clientele/latest/clientele/struct.StandardOptions.html
[`SysexitsError`]: https://docs.rs/clientele/latest/clientele/enum.SysexitsError.html
[`Utf8Path`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html
[`Utf8PathBuf`]: https://docs.rs/camino/latest/camino/struct.Utf8PathBuf.html
[`args_os()`]: https://docs.rs/clientele/latest/clientele/fn.args_os.html
[`dotenv()`]: https://docs.rs/clientele/latest/clientele/fn.dotenv.html
[`envs::*`]: https://docs.rs/getenv/latest/getenv/index.html
[`paths::*`]: https://docs.rs/clientele/latest/clientele/paths/index.html
