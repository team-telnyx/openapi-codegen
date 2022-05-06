//! Command line argument options and processing

use clap::Parser;

/// Generate well-typed Python HTTP API clients from an OpenAPI specification
///
/// The OpenAPI file is read from `stdin` and written to `stdout`.
#[derive(Parser)]
pub(crate) enum Args {
    /// Indicate that `stdin` is formatted as YAML
    Yaml,

    /// Indicate that `stdin` is formatted as JSON
    Json,
}
