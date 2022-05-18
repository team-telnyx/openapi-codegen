//! Struct parsing

use std::collections::BTreeMap;

use super::Field;

/// A parsed struct
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct {
    /// Documentation from OpenAPI assocaited with this structure
    pub docs: Option<String>,

    /// The fields of this structure
    ///
    /// The [`BTreeMap`](BTreeMap)'s keys are the names of the fields.
    pub fields: BTreeMap<String, Field>,
}

impl Struct {
    /// Add documentation to a [`Struct`](Struct)
    pub fn set_docs<S: Into<String>>(&mut self, docs: S) {
        self.docs = Some(docs.into());
    }
}

impl From<BTreeMap<String, Field>> for Struct {
    fn from(fields: BTreeMap<String, Field>) -> Self {
        Self {
            docs: None,
            fields,
        }
    }
}
