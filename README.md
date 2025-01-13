# Clientele.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.70%2B-blue)](https://rust-lang.org)
[![Package](https://img.shields.io/crates/v/clientele)](https://crates.io/crates/clientele)
[![Documentation](https://docs.rs/clientele/badge.svg)](https://docs.rs/clientele/)

**Clientele** makes it easy to write superb command-line utilities in Rust that
follow consistent best practices on all target platforms including Linux, macOS,
and Windows. It packages and re-exports [`clap`], [`camino`], [`dotenvy`],
[`argfile`], and [`wild`] into a single easy dependency.

## ✨ Features

- Showcases how to structure a CLI program in Rust (see the [examples](#-examples)).
- Loads environment variables from `.env` files (using the [`dotenvy`] crate).
- Expands wildcards (globs) on Windows (using the [`wild`] crate).
- Expands @argfiles similarly to [`javac`] or Python (using the [`argfile`] crate).
- Defines a standard set of essential CLI options (using the [`clap`] crate).
- Provides the [`Utf8Path`] and [`Utf8PathBuf`] types (using the [`camino`] crate).
- Recommends use of the [`sysexits.h(3)`] exit codes (see [`sysexits.rs`]).
- Supports opting out of any feature using comprehensive feature flags.
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.70+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add clientele
```

## 👉 Examples

See [`lib/clientele/examples/skeleton/main.rs`] for a complete example.

### Importing the library

```rust
use clientele::*;
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

## 👨‍💻 Development

```bash
git clone https://github.com/dryrust/clientele.rs.git
```

- - -

[![Share on Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?logo=twitter)](https://twitter.com/share?url=https://github.com/dryrust/clientele.rs&text=Clientele.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dryrust/clientele.rs&title=Clientele.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hacker%20news-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dryrust/clientele.rs&t=Clientele.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dryrust/clientele.rs)

[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[`StandardOptions`]: https://docs.rs/clientele/latest/clientele/struct.StandardOptions.html
[`Utf8Path`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html
[`Utf8PathBuf`]: https://docs.rs/camino/latest/camino/struct.Utf8PathBuf.html
[`javac`]: https://docs.oracle.com/javase/7/docs/technotes/tools/windows/javac.html#commandlineargfile
[`lib/clientele/examples/skeleton/main.rs`]: lib/clientele/examples/skeleton/main.rs

[`argfile`]: https://crates.io/crates/argfile
[`camino`]: https://crates.io/crates/camino
[`clap`]: https://crates.io/crates/clap
[`dotenvy`]: https://crates.io/crates/dotenvy
[`wild`]: https://crates.io/crates/wild

[`sysexits.h(3)`]: https://man7.org/linux/man-pages/man3/sysexits.h.3head.html
[`sysexits.rs`]: lib/clientele/src/sysexits.rs
