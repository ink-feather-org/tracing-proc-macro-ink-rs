#![allow(unused_imports)]
use proc_macro::TokenStream;

use tracing::{debug, debug_span, error, info, info_span, trace, trace_span, warn, warn_span};
use tracing_proc_macros_rs::proc_macro_logger_default_setup;

#[proc_macro_derive(DebuggableProcMacro)]
pub fn derive_debuggable_proc_macro(_input: TokenStream) -> TokenStream {
  // Setup the logger for the proc_macro
  // This must be called in every top-level proc_macro function
  //#[cfg(debug_assertions)] // The debug assertions cfg can be used to only log when building in debug mode
  proc_macro_logger_default_setup();

  trace!("Trace log message!");
  debug!("Debug log message!");
  info!("Info log message!");
  warn!("Warn log message!");
  // Error logs cause the compilation to fail
  // error!("Error log message!");

  trace_span!("my_trace_span").in_scope(|| {
    trace!("Trace log message in span!");
    debug!("Debug log message in span!");
    info!("Info log message in span!");
    warn!("Warn log message in span!");
    // Error logs cause the compilation to fail
    // error!("Error log message in span!");
  });

  TokenStream::default()
}
