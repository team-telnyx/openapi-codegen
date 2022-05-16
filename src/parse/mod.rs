//! Facilities for parsing OpenAPI schemas into meaningful structures

mod error;
mod field;
mod r#struct;
mod r#type;

pub use error::{Parse as Error, ParseKind as ErrorKind};
pub use field::Field;
pub use r#struct::Struct;
pub use r#type::Type;
