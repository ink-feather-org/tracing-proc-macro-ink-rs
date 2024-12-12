# `example_proc_macro`

This crate contains an example proc macro that uses `tracing` to emit log records.

To view the log records, run the following command:

```sh
cargo clean -p example_proc_macro
RUST_LOG=trace cargo build --workspace --bins
```

To compile without logs:

```sh
cargo clean -p example_proc_macro
cargo build --workspace --bins
```
