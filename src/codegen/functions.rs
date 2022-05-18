//! Generate code for HTTP methods

use std::fmt::Write;

use heck::ToSnakeCase;
use okapi::openapi3::OpenApi;

use super::type_to_string;
use crate::parse::Function;

/// Generates a function for each method available on each HTTP path
pub fn functions(openapi: &OpenApi) -> String {
    let fs = Function::try_from_paths(&openapi.paths).expect("problems");

    let mut code = String::new();

    for ((method, path), function) in fs {
        let fn_ident = format!("{method}_{path}").to_snake_case();

        let return_code = type_to_string(&function.responses.good, false);

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
    }

    code
}
