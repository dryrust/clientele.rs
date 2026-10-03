# Skeleton CLI

This example demonstrates Clientele's initialization sequence and standard
options. Its `config` subcommand prints a placeholder message; it does not read
or write an application configuration file.

## Run from the repository root

```sh
cargo run --locked --example skeleton -- --help
cargo run --locked --example skeleton -- config
cargo run --locked --example skeleton -- --version
cargo run --locked --example skeleton -- --license
```

Arguments after Cargo's `--` are passed to the example. Running with no arguments
shows usage and exits with status 2. To build with just the required features:

```sh
cargo run --locked --example skeleton --no-default-features --features clap,dotenv -- config
```

## Initialization

1. Load `.env` before processing arguments. Loading errors are ignored; existing
   environment variables retain precedence.
2. Collect OS-string arguments through `args_os()`. With `wild`, expand Windows
   globs first; with `argfile`, expand @argfiles afterward. Without these features,
   their respective expansion steps are skipped.
3. With `color`, inspect expanded arguments to configure Clap's help and error
   colors before parsing. This also covers `--color` supplied in an @argfile.
4. Parse the options and optional subcommand, preserving the configured Clap
   command for later diagnostics.
5. Handle `--version` and `--license` as successful early exits (version takes
   precedence if both are supplied). Clap handles help during parsing.
6. With `tracing`, initialize the global stderr subscriber from `StandardOptions`.
   The standalone application owns initialization; calling this entry point with
   a subscriber already installed panics. Embedded applications can use the
   fallible `try_init_tracing_subscriber` API instead.
7. Dispatch `config`, or report a missing subcommand after otherwise valid flags.
   Keep I/O errors intact for diagnostics, then select their sysexits status at
   the process boundary rather than collapsing every error into status 1.

## Features and options

The example requires `clap,dotenv`; `clap` also enables `std`. Default features
add argument-file expansion, Windows wildcard expansion, color, and tracing.
With `argfile`, an argument such as `@args.txt` loads arguments from that file
(one argument per line); a file containing `config` runs the subcommand.

`--color auto|always|never` is available with `color`; `auto` uses terminal and
environment detection. With `tracing`, `--debug` enables trace-level logging and
event metadata; repeatable `--verbose` selects warning (`-v`), info (`-vv`), or
debug (`-vvv` or more) levels with plain formatting. The default level is error.
`config` emits a debug event on stderr, visible with `--debug` or `-vvv`, while
its result remains on stdout. Without `tracing`, these flags are parsed but do
not initialize logging. Other optional library integrations are not used by
`config`.

```sh
cargo run --locked --example skeleton -- --debug config
cargo run --locked --example skeleton --no-default-features --features clap,dotenv,tracing -- -vvv config
```

Log color follows `--color` for stderr when `color` is enabled. Automatic color
requires a terminal and no nonempty `NO_COLOR`; explicit `always`/`never`
override detection. Without `color`, logs contain no ANSI color escapes.
The example uses the development dependency `tracing` for its event macro;
applications copying it should add their own `tracing` dependency.

## Exit statuses

| Invocation or outcome | Status | Output |
| --- | --- | --- |
| `config` | 0 | Placeholder message on stdout; debug log on stderr when enabled |
| `--help`, `--version`, `--license` | 0 | Requested information on stdout |
| No arguments, unknown option/subcommand, or invalid Clap value | 2 | Clap usage/diagnostic on stderr |
| Valid flags without a subcommand, e.g. `--debug` | 64 (`EX_USAGE`) | Missing-subcommand diagnostic on stderr |
| Missing @argfile, with `argfile` enabled | 66 (`EX_NOINPUT`) | Application error on stderr |

Argument-file errors include the filename, failed operation, and underlying
I/O cause, including missing files and invalid UTF-8 contents. Exit-code
classification does not replace that diagnostic with just a sysexits name.

The subprocess regression driver in `lib/clientele/tests/skeleton_cli.rs`
exercises the actual entry point with isolated environment settings:

```sh
cargo test -p clientele --test skeleton_cli --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,dotenv --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,color,dotenv --locked
```
