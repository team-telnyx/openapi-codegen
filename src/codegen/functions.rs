//! Generate code for HTTP methods

use std::collections::HashMap;

use heck::ToSnakeCase;
use okapi::openapi3::OpenApi;

use super::type_to_string;
use crate::parse::{Argument, Function, SecurityScheme, Type};

/// Generates a function for each method available on each HTTP path
// TODO: remove this when more HTTP auth methods are implemented
#[allow(clippy::zero_sized_map_values)]
pub fn functions(
    openapi: &OpenApi,
    security_schemes: &HashMap<String, SecurityScheme>,
) -> String {
    let fs = Function::try_from_paths(&openapi.paths).expect("problems");

    let mut code = String::new();

    for ((method, path), function) in fs {
        code.push_str(&signature(1, &method, &path, &function));
        code.push_str(&documentation(2, function.docs.as_ref()));
        code.push_str(&body(2, method, path, &function, security_schemes));
        code.push('\n');
    }

    code
}

/// Generate an amount of indentations
fn indents(indent_level: usize) -> String {
    (0..indent_level).into_iter().fold(String::new(), |mut acc, _| {
        acc.push_str(super::INDENT);
        acc
    })
}

/// Generate the body of a function
// TODO: remove this when more HTTP auth methods are implemented
#[allow(clippy::zero_sized_map_values)]
fn body<S1, S2>(
    indent_level: usize,
    method: S1,
    path: S2,
    function: &Function,
    security_schemes: &HashMap<String, SecurityScheme>,
) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let method = method.as_ref();
    let path = path.as_ref();

    // A list of methods this request can be authenticated by
    let mut schemes = function
        .security_schemes
        .iter()
        .filter_map(|name| security_schemes.get(name))
        .copied();

    let auth_args = if schemes.any(|x| x == SecurityScheme::BasicAuth) {
        "auth=self._auth"
    } else {
        ""
    };

    format!(
        "{i}resp = await \
         self._session.{method}(f\"{{self._base_url}}{path}\", {auth_args})",
        i = indents(indent_level),
    )
}

/// Generate the documentation for a function
fn documentation<S: AsRef<str>>(
    indent_level: usize,
    docs: Option<S>,
) -> String {
    match docs {
        Some(x) => {
            format!(
                "{i}\"\"\"\n{}\n{i}\"\"\"\n",
                x.as_ref(),
                i = indents(indent_level)
            )
        }
        None => format!("{i}\"\"\"\n{i}\"\"\"\n", i = indents(indent_level)),
    }
}

/// Generate an entire function signature
///
/// For example, `async def get_foo_foo_id(foo_id: str) -> Foo:`. This includes
/// the requested indentation and a single trailing newline.
fn signature<S1, S2>(
    indent_level: usize,
    method: S1,
    path: S2,
    function: &Function,
) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{i}async def {}(self, {}) -> {}:\n",
        name(method, path),
        arguments(function.arguments.iter()),
        return_type(&function.responses.responses),
        i = indents(indent_level),
    )
}

/// Generate the name of a function
fn name<S1, S2>(method: S1, path: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!("{}_{}", method.as_ref(), path.as_ref()).to_snake_case()
}

/// Generate the arguments that a function takes
///
/// These go between the `(` and `)`. Return value will not contain any
/// newlines.
fn arguments<'a, I: Iterator<Item = &'a Argument>>(arguments: I) -> String {
    arguments.fold(String::default(), |mut acc, x| {
        acc.push_str(&format!(
            "{}: {}, ",
            &x.name,
            type_to_string(&x.r#type, false)
        ));
        acc
    })
}

/// Generate the type that a function returns
///
/// This is the part that goes between the `->` and the `:`. Return value will
/// not contain any newlines.
fn return_type(responses: &HashMap<String, Type>) -> String {
    let return_types =
        responses.iter().map(|(_code, ty)| ty).cloned().collect::<Vec<_>>();

    match return_types.as_slice() {
        [] => String::from("None"),
        [x] => type_to_string(x, false),
        _ => {
            let mut return_code = return_types
                .into_iter()
                .map(|x| type_to_string(&x, false))
                .fold(String::from("Union["), |mut acc, x| {
                    acc.push_str(&x);
                    acc.push_str(", ");

                    acc
                });

            return_code.push(']');

            return_code
        }
    }
}
