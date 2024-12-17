# `example_proc_macro`

This crate contains an example proc macro that uses `tracing` to emit log records.

To view the log records, run the following command:

```sh
RUST_LOG=trace cargo build --workspace --bins
```

To compile without logs:

```sh
cargo build --workspace --bins
```
