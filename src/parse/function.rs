//! Function parsing

use std::collections::HashMap;

use okapi::{
    openapi3::{Operation, Parameter, ParameterValue, PathItem, RefOr},
    schemars::Map,
};

use super::{Error, ErrorKind, Type};

/// A parsed function
#[derive(Debug)]
pub struct Function {
    /// This function's documentation
    pub docs: Option<String>,

    /// The arguments this function will take
    pub arguments: Vec<Argument>,

    /// Names of security schemes this request can use
    pub security_schemes: Vec<String>,

    /// The responses returned by this API request
    pub responses: HashMap<String, Type>,
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
            eprintln!("{}\t{}", stringify!($method), $path);
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

        let responses = operation
            .responses
            .responses
            .iter()
            .map(|(code, response)| match response {
                RefOr::Object(x) => Ok((code, x)),
                RefOr::Ref(_) => Err(Error::from(ErrorKind::Unimplemented)),
            })
            .filter_map(|x| {
                x.map(|(code, x)| {
                    x.content.get("application/json").map(|x| (code, x))
                })
                .transpose()
            })
            .filter_map(|x| {
                x.map(|(code, x)| x.schema.as_ref().map(|x| (code, x)))
                    .transpose()
            })
            .map(|x| x.and_then(|(code, x)| Ok((code, Type::try_from(x)?))))
            .try_fold(HashMap::new(), |mut acc, x| {
                let (code, response) = x?;
                acc.insert(code.clone(), response);
                Ok::<_, Error>(acc)
            })?;

        Ok(Function {
            // TODO: include more things like examples, summary, and so on
            docs: operation.description.clone(),
            security_schemes: operation
                .security
                .iter()
                .flat_map(|x| x.iter().flat_map(|x| x.iter().map(|(k, _)| k)))
                .cloned()
                .collect(),
            arguments: args,

            responses,
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
                    Some((param, schema))
                } else {
                    // TODO: is this lossy?
                    None
                }
            })
            .try_fold(Vec::default(), |mut acc, (param, schema)| {
                acc.push(Argument {
                    name: param.name.clone(),
                    r#type: if param.required {
                        schema.try_into()?
                    } else {
                        Type::Option(Box::new(schema.try_into()?))
                    },
                });

                Ok::<_, Error>(acc)
            })
    }
}
