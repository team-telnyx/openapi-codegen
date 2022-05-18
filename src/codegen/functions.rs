//! Generate code for HTTP methods

use std::{collections::HashMap, fmt::Write};

use heck::ToSnakeCase;
use okapi::openapi3::OpenApi;

use super::type_to_string;
use crate::parse::{Function, SecurityScheme};

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
        let fn_ident = format!("{method}_{path}").to_snake_case();

        let return_code = {
            let return_types = function
                .responses
                .responses
                .iter()
                .map(|(_code, ty)| ty)
                .cloned()
                .collect::<Vec<_>>();

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
        };

        let args =
            function.arguments.iter().fold(String::default(), |mut acc, x| {
                write!(
                    &mut acc,
                    "{}: {}, ",
                    x.name,
                    type_to_string(&x.r#type, false)
                )
                .expect("write failed");

                acc
            });

        writeln!(
            &mut code,
            "{}async def {}(self, {}) -> {}:",
            super::INDENT,
            fn_ident,
            args,
            return_code,
        )
        .expect("write failed");

        if let Some(docs) = function.docs {
            writeln!(
                &mut code,
                "{i}{i}\"\"\"{}\"\"\"",
                docs,
                i = super::INDENT,
            )
            .expect("write failed");
        } else {
            writeln!(&mut code, "{i}{i}pass", i = super::INDENT)
                .expect("write failed");
        }

        // A list of methods this request can be authenticated by
        let mut schemes = function
            .security_schemes
            .iter()
            .filter_map(|name| security_schemes.get(name))
            .copied();

        write!(
            &mut code,
            "{i}{i}resp = await self._session.get(f\"{{self._base_url}}{}\", ",
            path,
            i = super::INDENT,
        )
        .expect("write failed");

        if schemes.any(|x| x == SecurityScheme::BasicAuth) {
            write!(&mut code, "auth=self._auth").expect("write failed");
        }

        writeln!(&mut code, ")\n").expect("write failed");
    }

    code
}
