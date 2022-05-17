//! Facilities for parsing OpenAPI schemas into meaningful structures

mod error;
mod field;
mod function;
mod r#struct;
mod r#type;

pub use error::{Parse as Error, ParseKind as ErrorKind};
pub use field::Field;
pub use function::Function;
pub use r#struct::Struct;
pub use r#type::Type;
