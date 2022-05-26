//! Generate code for HTTP methods

use std::{borrow::Borrow, collections::BTreeMap};

use heck::ToSnakeCase;
use okapi::openapi3::OpenApi;

use super::type_to_string;
use crate::parse::{Argument, Function, Location, SecurityScheme, Type};

/// Generates a function for each method available on each HTTP path
// TODO: remove this when more HTTP auth methods are implemented
#[allow(clippy::zero_sized_map_values)]
pub fn functions(
    openapi: &OpenApi,
    security_schemes: &BTreeMap<String, SecurityScheme>,
) -> String {
    let fs = Function::try_from_paths(&openapi.paths).expect("problems");

    let mut code = String::new();

    for ((method, path), function) in fs {
        code.push_str(&signature(1, &method, &path, &function));
        code.push_str(&documentation(2, function.docs.as_ref()));
        code.push_str(&body(2, method, path, &function, security_schemes));
        code.push_str("\n\n");
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

/// Generate any URL query parameters required to make the HTTP request
///
/// The first string in the tuple is the code that builds the query, and the
/// second string is what should be included in the HTTP call.
fn query_param_arguments(
    indent_level: usize,
    function: &Function,
) -> Option<(String, &'static str)> {
    let has_query_arguments =
        function.arguments.iter().any(|x| x.location == Location::Query);

    if !has_query_arguments {
        return None;
    }

    let mut code = String::new();

    code.push_str(&format!(
        "{i}init_params: List[Tuple[str, Optional[str]]] = [\n",
        i = indents(indent_level)
    ));

    // Generate a list of trivially convertable parameters
    function
        .arguments
        .iter()
        .filter(|x| x.location == Location::Query)
        .filter_map(|x| {
            var_to_url_str(&x.name, &x.r#type).map(|s| (&x.name, s))
        })
        .for_each(|(name, stringifier)| {
            code.push_str(&format!(
                "{i}(\"{name}\", {stringifier}),\n",
                i = indents(indent_level + 1)
            ));
        });

    // Close off the list
    code.push_str(&format!("{i}]\n", i = indents(indent_level)));

    // Omit any `None` values
    code.push_str(&format!(
        "{i}params: List[Tuple[str, str]] = [(k, v) for k, v in init_params \
         if v is not None]\n",
        i = indents(indent_level),
    ));

    // Convert lists and optional lists into tuple pairs
    //
    // I swear, if some OpenAPI spec takes a *list* of *options*...
    function
        .arguments
        .iter()
        .filter(|x| x.location == Location::Query)
        .filter_map(
            // TODO: This is probably fine, but ideally it would recurse. If it
            // does recurse, the `if {name} is not None` check below should
            // technically also recurse, but since `Option[T]` is the same as
            // `Union[T, None]` and `Union[Union[T, None] None]` is technically
            // the same, it probably doesn't actually matter.
            |x| match &x.r#type {
                Type::List(y) | Type::Set(y) => Some((&x.name, &x.r#type, y)),
                Type::Option(y) => match y.borrow() {
                    Type::List(y) | Type::Set(y) => {
                        Some((&x.name, &x.r#type, y))
                    }
                    _ => None,
                },
                _ => None,
            },
        )
        .filter_map(|(x, original_ty, list_item_ty)| {
            var_to_url_str("x", list_item_ty).map(move |s| (x, original_ty, s))
        })
        .for_each(|(name, ty, stringifier)| {
            if matches!(ty.borrow(), Type::Option(_)) {
                code.push_str(&format!(
                    "{i}if {name} is not None:\n",
                    i = indents(indent_level)
                ));
            }

            code.push_str(&format!(
                "{i}params.extend([(\"{name}\", {}) for x in {name}])\n",
                stringifier,
                name = name,
                i = indents(
                    indent_level
                        + if matches!(ty.borrow(), Type::Option(_)) {
                            1
                        } else {
                            0
                        }
                ),
            ));
        });

    code.push('\n');

    Some((code, "params=params, "))
}

/// From a variable, construct conversion code for turning it into a URL string
///
/// If this function returns `None`, there is no trivial conversion method.
fn var_to_url_str<S: AsRef<str>>(name: S, ty: &Type) -> Option<String> {
    let name = name.as_ref();

    match ty {
        // Trivial string conversion
        Type::String | Type::Float | Type::Integer => {
            Some(format!("str({name})"))
        }

        // Use the more typical lowercase versions
        Type::Bool => Some(format!(r#"("true" if {} else "false")"#, name)),

        // Handle optional values properly
        Type::Option(ty) => Some(format!(
            "({} if {name} is not None else None)",
            var_to_url_str(name, ty)?,
        )),

        _ => None,
    }
}

/// Generate the body of a function
// TODO: remove this when more HTTP auth methods are implemented
#[allow(clippy::zero_sized_map_values)]
fn body<S1, S2>(
    indent_level: usize,
    method: S1,
    path: S2,
    function: &Function,
    security_schemes: &BTreeMap<String, SecurityScheme>,
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
        "auth=self._auth, "
    } else {
        ""
    };

    let mut code = String::new();

    let param_args = if let Some((builder, args)) =
        query_param_arguments(indent_level, function)
    {
        code.push_str(&builder);
        args
    } else {
        ""
    };

    let body_args =
        if function.arguments.iter().any(|x| x.location == Location::Body) {
            "json=body.dict(by_alias=True), "
        } else {
            ""
        };

    code.push_str(&format!(
        "{i}resp = await \
         self._session.{method}(f\"{{self._base_url}}{path}\", \
         {auth_args}{param_args}{body_args})\n",
        i = indents(indent_level),
    ));

    if !function.responses.is_empty() {
        // Add some visual space
        code.push('\n');
    }

    function.responses.iter().for_each(|(status, ty)| {
        let cond = format!(
            "{i}if resp.status == {status}:\n",
            i = indents(indent_level)
        );

        let (_code, rets) = return_type(&function.responses);

        let body = match rets {
            Return::Many => format!(
                "{i}return (\"{ty}\", parse_obj_as({ty}, await resp.json()))\n",
                ty = type_to_string(ty, false),
                i = indents(indent_level + 1)
            ),
            Return::One => format!(
                "{i}return parse_obj_as({ty}, await resp.json())\n",
                ty = type_to_string(ty, false),
                i = indents(indent_level + 1)
            ),
        };

        code.push_str(&cond);
        code.push_str(&body);
    });

    if function.responses.is_empty() {
        code.push_str(&format!(
            "\n{i}resp.raise_for_status()",
            i = indents(indent_level)
        ));
    } else {
        code.push_str(&format!(
            "\n{i}raise aiohttp.ClientResponseError(resp.request_info, \
             (resp,), status=resp.status)",
            i = indents(indent_level)
        ));
    }

    code
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
        arguments(&function.arguments),
        return_type(&function.responses).0,
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
fn arguments(arguments: &[Argument]) -> String {
    let mut args = String::new();

    // Body argument goes first
    arguments.iter().filter(|x| x.location == Location::Body).for_each(|x| {
        args.push_str(&format!(
            "{}: {}, ",
            &x.name,
            type_to_string(&x.r#type, false)
        ));
    });

    // Other non-optional arguments go next
    arguments
        .iter()
        .filter(|x| {
            x.location != Location::Body
                && (!matches!(x.r#type, Type::Option(_)))
        })
        .for_each(|x| {
            args.push_str(&format!(
                "{}: {}, ",
                &x.name,
                type_to_string(&x.r#type, false)
            ));
        });

    // Optional arguments go last, defaulted to `None`
    arguments
        .iter()
        .filter(|x| {
            x.location != Location::Body && matches!(x.r#type, Type::Option(_))
        })
        .for_each(|x| {
            args.push_str(&format!(
                "{}: {} = None, ",
                &x.name,
                type_to_string(&x.r#type, false)
            ));
        });

    args
}

/// The amount of unique types a function can return
enum Return {
    /// Returns one type
    One,

    /// Returns multiple types
    Many,
}

/// Generate the type that a function returns
///
/// This is the part that goes between the `->` and the `:`. Return value will
/// not contain any newlines.
fn return_type(responses: &BTreeMap<String, Type>) -> (String, Return) {
    let return_types =
        responses.iter().map(|(_code, ty)| ty).cloned().collect::<Vec<_>>();

    match return_types.as_slice() {
        [] => (String::from("None"), Return::One),
        [x] => (type_to_string(x, false), Return::One),
        _ => {
            let mut return_code = return_types
                .into_iter()
                .map(|x| type_to_string(&x, false))
                .fold(String::from("Union["), |mut acc, x| {
                    acc.push_str(&format!(
                        "Tuple[Literal[\"{ty}\"], {ty}], ",
                        ty = x
                    ));

                    acc
                });

            return_code.push(']');

            (return_code, Return::Many)
        }
    }
}
