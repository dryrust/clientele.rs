# Downstream consumer fixture

This independent package depends on Clientele without a direct Clap dependency.
It compiles the actual skeleton source and runs the public derive examples as
doctests extracted by `build.rs`, so library-only extern-prelude visibility cannot
hide missing module imports. Its `color` and `tracing` features forward to
Clientele; tracing event macros use the documented direct `tracing` dependency.

Run from the repository root:

```sh
cargo test --manifest-path tests/consumer/Cargo.toml --locked --target-dir target
cargo test --manifest-path tests/consumer/Cargo.toml --all-features --locked --target-dir target
```

Cargo maintains this package's separate lockfile. Refresh it with
`cargo update --manifest-path tests/consumer/Cargo.toml --workspace --offline`
when the path dependency's workspace version changes.
