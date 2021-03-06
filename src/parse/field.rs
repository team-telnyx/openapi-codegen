//! Struct field parsing

use okapi::{openapi3::SchemaObject, schemars::schema::Schema};

use super::{Error, Type};

/// A parsed field
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Field {
    /// Type of the field
    pub r#type: Type,

    /// Field documentation
    pub docs: Option<String>,

    /// Whether the use of this field is deprecated
    pub deprecated: bool,
}

impl Field {
    /// Create a new instance of field data from the type of the data
    pub fn new(r#type: Type) -> Self {
        Self {
            r#type,
            docs: None,
            deprecated: false,
        }
    }

    /// Add documentation to a [`Field`](Field)
    pub fn set_docs<S: Into<String>>(&mut self, docs: S) {
        self.docs = Some(docs.into());
    }
}

impl TryFrom<&Schema> for Field {
    type Error = Error;

    fn try_from(schema: &Schema) -> Result<Self, Self::Error> {
        match schema {
            Schema::Bool(_) => todo!(),
            Schema::Object(x) => x.try_into(),
        }
    }
}

impl TryFrom<&SchemaObject> for Field {
    type Error = Error;

    fn try_from(schema_object: &SchemaObject) -> Result<Self, Self::Error> {
        let mut x = Field::new(Type::try_from(schema_object)?);

        if let Some(docs) =
            schema_object.metadata.as_ref().and_then(|x| x.description.as_ref())
        {
            // TODO: include more things like examples, the title, and so on
            x.set_docs(docs);
        }

        if schema_object.metadata.as_ref().map_or(false, |x| x.deprecated) {
            x.r#type = Type::Option(Box::new(x.r#type));
            x.deprecated = true;

            if let Some(x) = x.docs.as_mut() {
                x.push_str("\n\nThis field is deprecated.");
            } else {
                x.docs = Some("This field is deprecated".to_owned());
            }
        }

        Ok(x)
    }
}
