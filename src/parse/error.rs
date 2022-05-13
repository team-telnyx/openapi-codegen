//! Errors that can occur during parsing

use std::fmt;

use backtrace::Backtrace;
use thiserror::Error;

/// An error converting JSON Schema to strongly-typed data structures
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Parse {
    /// Backtrace for easier debugging
    pub backtrace: Backtrace,

    /// The actual error information
    pub kind: ParseKind,
}

impl From<ParseKind> for Parse {
    fn from(other: ParseKind) -> Self {
        Self {
            kind: other,
            backtrace: Backtrace::new(),
        }
    }
}

impl fmt::Display for Parse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl std::error::Error for Parse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

/// Errors converting JSON Schema to strongly-typed data structures
#[derive(Debug, Error)]
pub enum ParseKind {
    /// The type description is currently unsupported
    #[error("the type description is currently unsupported")]
    Unimplemented,

    /// The calling function is not reponsible for processing the current type
    ///
    /// This happens when, for example, `Type::try_from_bool` is asked to parse
    /// something that turns out to be an integer.
    #[error("this object is unsupported by this function")]
    OtherType,
}
