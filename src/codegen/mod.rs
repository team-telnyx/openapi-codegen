//! Facilities for generating source code

/// Constant containing the whitespace to be used for indentation
const INDENT: &str = "    ";

mod methods;
pub use methods::methods;

mod module;
pub use module::module;

mod types;
pub use types::types;
