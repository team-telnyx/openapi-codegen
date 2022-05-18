//! Struct parsing

use std::collections::HashMap;

use super::Field;

/// A parsed struct
#[derive(Debug, PartialEq, Eq)]
pub struct Struct {
    /// Documentation from OpenAPI assocaited with this structure
    pub docs: Option<String>,

    /// The fields of this structure
    ///
    /// The [`HashMap`](HashMap)'s keys are the names of the fields.
    pub fields: HashMap<String, Field>,
}

impl Struct {
    /// Add documentation to a [`Struct`](Struct)
    pub fn set_docs<S: Into<String>>(&mut self, docs: S) {
        self.docs = Some(docs.into());
    }
}

impl From<HashMap<String, Field>> for Struct {
    fn from(fields: HashMap<String, Field>) -> Self {
        Self {
            docs: None,
            fields,
        }
    }
}
