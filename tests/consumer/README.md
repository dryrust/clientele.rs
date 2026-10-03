# Downstream consumer fixture

This independent package depends on Clientele without a direct Clap dependency.
It compiles the actual skeleton source and runs the public derive examples as
doctests extracted by `build.rs`, so library-only extern-prelude visibility cannot
hide missing module imports. Its `color` and `tracing` features forward to
Clientele; tracing event macros use the documented direct `tracing` dependency.
The default `cli` feature enables the derive examples. Disabling it allows the
scoped-subscriber example to use only Clientele's `std,tracing` features, with no
Clap or direct `tracing-subscriber` dependency.

Run from the repository root:

```sh
cargo test --manifest-path tests/consumer/Cargo.toml --locked --target-dir target
cargo test --manifest-path tests/consumer/Cargo.toml --all-features --locked --target-dir target
cargo run --manifest-path tests/consumer/Cargo.toml --no-default-features --features tracing --example scoped_tracing --locked --target-dir target
```

Cargo maintains this package's separate lockfile. Refresh it with
`cargo update --manifest-path tests/consumer/Cargo.toml --workspace --offline`
when the path dependency's workspace version changes.
