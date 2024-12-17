# tracing-proc-macros-ink

[![docs.rs](https://docs.rs/tracing-proc-macros-ink/badge.svg)](https://docs.rs/tracing-proc-macros-ink)
[![crates.io](https://img.shields.io/crates/v/tracing-proc-macros-ink.svg)](https://crates.io/crates/tracing-proc-macros-ink)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

`tracing-proc-macros-ink` provides a working tracing integration for logging from within procedural macros.
It only works in `proc-macro = true` crates and is nightly only.

Log records are emitted using nightly compiler diagnostics.

## Requirements

This crate requires a nightly compiler.

## Usage

Every top level function in your `proc_macro` crate should call `tracing-proc-macros-ink::proc_macro_logger_default_setup()` to setup the logger for the `proc_macro` crate.
After that normal `tracing` logging can be used in the `proc_macro` crate.

## Example

Check out the example crate [`example_proc_macro`](./examples/example_proc_macro/).

## How to turn on logging?

By default logging is turned off.

To run it on, you need to set the `RUST_LOG` environment variable:

```sh
RUST_LOG=trace cargo build --workspace --bins
```

## How to speed up the compilation?

To remove the logging calls entirely from the `proc_macro` crate, you can directly depend on `tracing` and enable the [features that remove the logging calls](https://docs.rs/tracing/latest/tracing/level_filters/index.html).

## Custom Logger setup

Providing a customized logging setup is trivial.
Copy the code from `tracing-proc-macros-ink::proc_macro_logger_default_setup()` into your own crate and modify it to your needs.
The default `default-setup` feature can be disabled with `no-default-features = true` to avoid enabling unnecessary `tracing-subscriber` features.

## License

This project is released under either:

- [MIT License](https://github.com/ink-feather-org/tracing-proc-macros-ink-rs/blob/main/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/ink-feather-org/tracing-proc-macros-ink-rs/blob/main/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

### Links

[`tracing`](https://crates.io/crates/tracing)

[`tracing-subscriber`](https://crates.io/crates/tracing-subscriber)
