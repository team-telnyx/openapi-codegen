//! Type parsing
//!
//! "Type" meaning both primitive JSON types like strings and numbers as well as
//! complex types like objects and lists.

use std::collections::HashMap;

use okapi::{
    openapi3::SchemaObject,
    schemars::schema::{InstanceType, Schema, SingleOrVec},
};

use super::{Error, ErrorKind, Field, Struct};

/// Representation of a data structure suitable for codegen
///
/// The preferred way to construct this type is to call its
/// [`TryFrom`](TryFrom)`<&`[`SchemaObject`](SchemaObject)`>` implementation.
// TODO: include validation/formatting information
#[derive(Debug)]
pub enum Type {
    /// Character sequence
    String,

    /// Nothing
    None,

    /// True or false
    Bool,

    /// Decimal number
    Float,

    /// Whole number
    Integer,

    /// Any type
    Any,

    /// List of another [`Type`](Type)
    List(Box<Self>),

    /// Deduplicated list of another [`Type`](Type)
    Set(Box<Self>),

    /// A collection of properties
    Struct(Struct),

    /// Name of another type
    Ref(String),
}

impl From<Struct> for Type {
    fn from(x: Struct) -> Self {
        Self::Struct(x)
    }
}

impl TryFrom<&SchemaObject> for Type {
    type Error = Error;

    fn try_from(schema_object: &SchemaObject) -> Result<Self, Self::Error> {
        // Possible data types:
        //
        // * [ ] enum?
        // * [?] ref-to-object
        // * [X] array
        // * [X] boolean
        // * [X] integer
        // * [X] null
        // * [X] number
        // * [X] object
        // * [X] string

        Self::try_from_ref(schema_object)
            .or_else(|_| Self::try_from_object(schema_object))
            .or_else(|_| Self::try_from_null(schema_object))
            .or_else(|_| Self::try_from_string(schema_object))
            .or_else(|_| Self::try_from_number(schema_object))
            .or_else(|_| Self::try_from_integer(schema_object))
            .or_else(|_| Self::try_from_boolean(schema_object))
            .or_else(|_| Self::try_from_array(schema_object))
    }
}

/// Generate a simple conversion function from [`SchemaObject`](SchemaObject)s
/// to [`Type`](Type)s
macro_rules! try_from_simple {
    (
        $($fn_name:ident = $their_variant:ident => $our_variant:ident),* $(,)?
    ) => {
        $(
            /// Try to convert a [`SchemaObject`](SchemaObject) into a simple
            /// type
            fn $fn_name(
                schema_object: &SchemaObject,
            ) -> Result<Self, Error> {
                let single_or_vec = schema_object
                    .instance_type
                    .as_ref()
                    .ok_or(ErrorKind::Unimplemented)?;

                let instance_type = match single_or_vec {
                    SingleOrVec::Single(x) => x,
                    _ => return Err(ErrorKind::Unimplemented.into()),
                };

                match instance_type.as_ref() {
                    InstanceType::$their_variant => Ok(Self::$our_variant),
                    _ => Err(ErrorKind::OtherType.into()),
                }
            }
        )*
    };
}

impl Type {
    try_from_simple! {
        try_from_string = String => String,
        try_from_number = Number => Float,
        try_from_integer = Integer => Integer,
        try_from_boolean = Boolean => Bool,
        try_from_null = Null => None,
    }

    /// Try to convert a [`SchemaObject`](SchemaObject) into a ref-to-type
    fn try_from_ref(schema_object: &SchemaObject) -> Result<Self, Error> {
        if let Some(reference) = schema_object.reference.as_deref() {
            Ok(Self::Ref(reference.to_owned()))
        } else {
            Err(ErrorKind::OtherType.into())
        }
    }

    /// Try to convert a [`SchemaObject`](SchemaObject) into an object type
    fn try_from_object(schema_object: &SchemaObject) -> Result<Self, Error> {
        let x = schema_object
            .object
            .as_deref()
            .ok_or_else(|| ErrorKind::Unimplemented.into())
            .and_then(|object_validation| {
                let mut s: Struct = object_validation
                    .properties
                    .iter()
                    // TODO: use `Iterator::try_collect` instead when it stabilizes
                    .try_fold(HashMap::new(), |mut acc, (name, schema)| {
                        // TODO: handle case where field type is an object
                        acc.insert(name.clone(), Field::try_from(schema)?);
                        Ok::<_, Error>(acc)
                    })?
                    .into();

                if let Some(docs) = schema_object
                    .metadata
                    .as_ref()
                    .and_then(|x| x.description.as_ref())
                {
                    // TODO: include more things like examples, the title, and
                    // so on
                    s.set_docs(docs);
                }

                Ok(Type::Struct(s))
            });

        x
    }

    /// Try to convert a [`SchemaObject`](SchemaObject) into list-like type
    ///
    /// "List-like" meaning things like [`Vec`](Vec) and
    /// [`HashSet`](std::collections::HashSet).
    fn try_from_array(schema_object: &SchemaObject) -> Result<Self, Error> {
        let single_or_vec = schema_object
            .instance_type
            .as_ref()
            .ok_or(ErrorKind::Unimplemented)?;

        let instance_type = if let SingleOrVec::Single(x) = single_or_vec {
            x
        } else {
            return Err(ErrorKind::Unimplemented.into());
        };

        match instance_type.as_ref() {
            InstanceType::Array => (),
            _ => return Err(ErrorKind::OtherType.into()),
        }

        let array_validation = match schema_object.array.as_ref() {
            Some(x) => x,

            // There's either no type information, or it's specified in a way we
            // don't understand. So, we have to assume that it can be any valid
            // JSON.
            None => return Ok(Type::Any),
        };

        match &array_validation.items {
            None | Some(SingleOrVec::Vec(_)) => {
                Err(ErrorKind::Unimplemented.into())
            }
            Some(SingleOrVec::Single(schema)) => match schema.as_ref() {
                Schema::Bool(_) => todo!(),
                Schema::Object(schema_object) => {
                    let inner = Self::try_from(schema_object)?;

                    match array_validation.unique_items {
                        Some(true) => Ok(Self::Set(Box::new(inner))),

                        // Assume unspecified means regular list
                        None | Some(false) => Ok(Self::List(Box::new(inner))),
                    }
                }
            },
        }
    }
}
