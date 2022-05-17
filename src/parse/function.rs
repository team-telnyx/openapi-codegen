//! Function parsing

use std::collections::HashMap;

use okapi::{
    openapi3::{Operation, Parameter, ParameterValue, PathItem, RefOr},
    schemars::Map,
};

use super::{Error, Type};

/// A parsed function
#[derive(Debug)]
pub struct Function {
    /// This function's documentation
    pub docs: Option<String>,

    /// The arguments this function will take
    pub arguments: Vec<Argument>,
}

/// An owned HTTP method
///
/// For example, `GET`, `POST`, `DELETE`, and so on.
pub type HttpMethodBuf = String;

/// An owned OpenAPI HTTP path
///
/// For example, `/foo/{bar}`, where `{bar}` represents a path argument.
pub type OpenApiPathBuf = String;

/// Functions generated from OpenAPI
type Functions = HashMap<(HttpMethodBuf, OpenApiPathBuf), Function>;

/// Deduplicates HTTP method specification
macro_rules! parse_function {
    ($functions:ident, $path:ident, $path_item:ident, $method:ident) => {
        if let Some(operation) = $path_item.$method.as_ref() {
            $functions.insert(
                (stringify!($method).to_owned(), $path.to_owned()),
                Self::try_from_operation(operation)?,
            );
        }
    };
}

impl Function {
    /// Generate a map of HTTP paths to function signatures from OpenAPI data
    pub fn try_from_paths(
        paths: &Map<OpenApiPathBuf, PathItem>,
    ) -> Result<Functions, Error> {
        paths.iter().try_fold(Functions::default(), |mut acc, (path, info)| {
            parse_function!(acc, path, info, get);
            parse_function!(acc, path, info, put);
            parse_function!(acc, path, info, post);
            parse_function!(acc, path, info, delete);
            parse_function!(acc, path, info, options);
            parse_function!(acc, path, info, head);
            parse_function!(acc, path, info, patch);
            parse_function!(acc, path, info, trace);

            Ok(acc)
        })
    }

    /// Generates a method for a given HTTP URL and HTTP method
    fn try_from_operation(operation: &Operation) -> Result<Self, Error> {
        let args = Argument::try_from_parameters(operation.parameters.iter())?;

        Ok(Function {
            // TODO: include more things like examples, summary, and so on
            docs: operation.description.clone(),
            arguments: args,
        })
    }
}

/// A parsed function argument
#[derive(Debug)]
pub struct Argument {
    /// The name of this argument
    pub name: String,

    /// The type of this argument
    pub r#type: Type,
}

impl Argument {
    /// Construct a list of arguments out of HTTP path arguments
    fn try_from_parameters<'a, I>(ref_or_params: I) -> Result<Vec<Self>, Error>
    where
        I: Iterator<Item = &'a RefOr<Parameter>>,
    {
        ref_or_params
            .filter_map(|ref_or_param| {
                if let RefOr::Object(param) = ref_or_param {
                    Some(param)
                } else {
                    // TODO: is this lossy?
                    None
                }
            })
            .filter_map(|param| {
                if let ParameterValue::Schema {
                    schema,
                    ..
                } = &param.value
                {
                    Some((param.name.as_str(), schema))
                } else {
                    // TODO: is this lossy?
                    None
                }
            })
            .try_fold(Vec::default(), |mut acc, (name, schema)| {
                acc.push(Argument {
                    name: name.to_owned(),
                    r#type: schema.try_into()?,
                });

                Ok::<_, Error>(acc)
            })
    }
}
