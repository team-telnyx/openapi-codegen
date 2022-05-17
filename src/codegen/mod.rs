//! Facilities for generating source code

/// Constant containing the whitespace to be used for indentation
const INDENT: &str = "    ";

mod functions;
pub use functions::functions;

mod module;
pub use module::module;

mod types;
pub use types::{type_to_string, types};
