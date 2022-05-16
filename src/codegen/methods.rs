//! Generate code for HTTP methods

use heck::ToSnakeCase;
use okapi::openapi3::{OpenApi, Operation};

/// Generate a method signature for the given HTTP method and HTTP path
fn signature(method: &str, path: &str, operation: &Operation) -> String {
    let mut path_args = Vec::new();

    let mut path_fmt = String::new();

    let fn_name = path
        .split('/')
        .fold(String::new(), |mut acc, path_segment| {
            if path_segment.is_empty() {
                return acc;
            }

            if let Some(path_param_name) =
                path_segment.strip_prefix('{').and_then(|x| x.strip_suffix('}'))
            {
                acc.push('_');
                acc.push_str(path_param_name);

                path_args.push(format!(
                    "{}: {}",
                    &path_param_name.to_snake_case(),
                    crate::resolve_type_for(path_param_name, operation)
                ));

                path_fmt.push_str("/{}");
            } else {
                acc.push('_');
                acc.push_str(path_segment);

                path_fmt.push('/');
                path_fmt.push_str(path_segment);
            }

            acc
        })
        .to_snake_case();

    let fn_name = format!("{}_{}", method, fn_name);

    let mut sig = format!("async def {fn_name}(self, ");

    for path_arg in path_args {
        sig.push_str(&path_arg);
        sig.push_str(", ");
    }

    // TODO: return type
    sig.push_str("):");

    sig
}

/// Generate documentation for a method
fn docs(op: &Operation) -> Option<String> {
    (op.summary.is_some() || op.description.is_some()).then(|| {
        let mut docs = r#"""""#.to_owned();

        if let Some(summary) = op.summary.as_deref() {
            docs.push_str(summary);

            if op.description.is_some() {
                docs.push_str("\n\n");
            }
        }

        if let Some(description) = op.description.as_deref() {
            docs.push_str(description);
        }

        docs.push_str(r#"""""#);

        docs
    })
}

/// Generates a method for a given HTTP URL and HTTP method
fn method(
    tokens: &mut String,
    path: &str,
    operation: &Operation,
    method: &str,
) {
    let signature = signature(method, path, operation);

    tokens.push_str(super::INDENT);
    tokens.push_str(&signature);
    tokens.push('\n');

    if let Some(docs) = docs(operation) {
        tokens.push_str(super::INDENT);
        tokens.push_str(super::INDENT);
        tokens.push_str(&docs);
        tokens.push('\n');
    }

    // TODO: function body
    tokens.push_str(super::INDENT);
    tokens.push_str(super::INDENT);
    tokens.push_str(&format!(
        r#"self._session.{}(f"{{self._base_url}}{}")"#,
        method, path
    ));

    tokens.push_str("\n\n");
}

/// Deduplicates HTTP method specification
macro_rules! call_method {
    ($tokens:ident, $path:ident, $path_item:ident, $method:ident) => {
        if let Some(operation) = $path_item.$method.as_ref() {
            method(&mut $tokens, $path, operation, stringify!($method));
        }
    };
}

/// Generates a function for each method available on each HTTP path
pub fn methods(openapi: &OpenApi) -> String {
    openapi.paths.iter().fold(String::new(), |mut acc, (path, info)| {
        call_method!(acc, path, info, get);
        call_method!(acc, path, info, put);
        call_method!(acc, path, info, post);
        call_method!(acc, path, info, delete);
        call_method!(acc, path, info, options);
        call_method!(acc, path, info, head);
        call_method!(acc, path, info, patch);
        call_method!(acc, path, info, trace);

        acc
    })
}
