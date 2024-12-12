#![doc = include_str!("../README.md")]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use std::{io, sync::Once};

use proc_macro::Diagnostic;
use tracing::Level;
use tracing_subscriber::{
  fmt::{self, MakeWriter, time},
  prelude::*,
};

/// Converts a `tracing` log level to a `proc_macro` log level.
const fn convert_level(level: Level) -> proc_macro::Level {
  match level {
    Level::ERROR => proc_macro::Level::Error,
    Level::WARN => proc_macro::Level::Warning,
    Level::INFO => proc_macro::Level::Note,
    Level::DEBUG | Level::TRACE => proc_macro::Level::Help,
  }
}

/// A [`io::Write`] implementation that writes `tracing` logs to `proc_macro` diagnostics.
/// Users should not use this directly, but instead pass an instance of [`RustcDiagnosticsMakeWriter`] to [`fmt::Layer::with_writer`].
pub struct RustcDiagnosticsWriter {
  level: Level,
}

impl RustcDiagnosticsWriter {
  const fn new(level: Level) -> Self {
    Self { level }
  }
}

impl io::Write for RustcDiagnosticsWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    let message = String::from_utf8_lossy(buf);
    Diagnostic::new(convert_level(self.level), message).emit();
    Ok(buf.len())
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

/// A [`MakeWriter`] implementation that writes `tracing` logs to `proc_macro` diagnostics.
pub struct RustcDiagnosticsMakeWriter;

impl<'a> MakeWriter<'a> for RustcDiagnosticsMakeWriter {
  type Writer = RustcDiagnosticsWriter;

  fn make_writer(&'a self) -> Self::Writer {
    RustcDiagnosticsWriter::new(Level::INFO)
  }

  fn make_writer_for(&'a self, meta: &tracing::Metadata<'_>) -> Self::Writer {
    let level = *meta.level();
    RustcDiagnosticsWriter::new(level)
  }
}

/// Sets up a default logging arrangement for the `proc_macro`.
/// This function should be called once in every top level function of your `proc_macro` crate.
/// To use a custom logging setup take a look at the source code of this function.
///
/// When writing your own setup function, make sure to use a `static SETUP_LOGGER: Once = Once::new();` to ensure that the setup is only done once.
#[cfg(feature = "default-setup")]
pub fn proc_macro_logger_default_setup() {
  static SETUP_LOGGER: Once = Once::new();
  SETUP_LOGGER.call_once(|| {
    const FILE_INFO: bool = true;
    tracing_subscriber::registry()
      .with(
        fmt::Layer::default()
          .with_ansi(false)
          .with_file(FILE_INFO)
          .with_line_number(FILE_INFO)
          .with_timer(time::OffsetTime::local_rfc_3339().expect("Could not get local offset!"))
          .with_writer(RustcDiagnosticsMakeWriter),
      )
      .with(tracing_subscriber::EnvFilter::from_default_env())
      .init();
  });
}
