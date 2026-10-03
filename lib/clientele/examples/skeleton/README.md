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
6. Dispatch `config`, or report a missing subcommand after otherwise valid flags.
   Return application errors through their sysexits status rather than collapsing
   every error into status 1.

## Features and options

The example requires `clap,dotenv`; `clap` also enables `std`. Default features
add argument-file expansion, Windows wildcard expansion, and color support.
With `argfile`, an argument such as `@args.txt` loads arguments from that file
(one argument per line); a file containing `config` runs the subcommand.

`--color auto|always|never` is available with `color`; `auto` uses terminal and
environment detection. `--debug` and repeatable `--verbose` are parsed as
standard flags. This version of the example does not initialize a logging
subscriber yet. Other optional library integrations are not used by `config`.

## Exit statuses

| Invocation or outcome | Status | Output |
| --- | --- | --- |
| `config` | 0 | Placeholder message on stdout |
| `--help`, `--version`, `--license` | 0 | Requested information on stdout |
| No arguments, unknown option/subcommand, or invalid Clap value | 2 | Clap usage/diagnostic on stderr |
| Valid flags without a subcommand, e.g. `--debug` | 64 (`EX_USAGE`) | Missing-subcommand diagnostic on stderr |
| Missing @argfile, with `argfile` enabled | 66 (`EX_NOINPUT`) | Application error on stderr |

The subprocess regression driver in `lib/clientele/tests/skeleton_cli.rs`
exercises the actual entry point with isolated environment settings:

```sh
cargo test -p clientele --test skeleton_cli --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,dotenv --locked
cargo test -p clientele --test skeleton_cli --no-default-features --features clap,color,dotenv --locked
```
