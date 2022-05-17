//! Generate the API client module

use okapi::openapi3::OpenApi;

/// Generate the API client module
pub fn module(openapi: &OpenApi) -> String {
    let mut module = String::new();

    let module_docs = {
        let mut module_docs = format!("{} HTTP API client", openapi.info.title);

        if let Some(description) = &openapi.info.description {
            module_docs.push_str("\n\n");
            module_docs.push_str(description);
        };

        format!(r#""""{module_docs}""""#)
    };

    module.push_str(&module_docs);
    module.push_str("\n\n");

    module.push_str(include_str!("imports.py"));
    module.push_str("\n\n");

    module.push_str(&crate::codegen::types(openapi));

    module.push_str(include_str!("api_client.py"));
    module.push_str("\n\n");

    module.push_str(&crate::codegen::functions(openapi));

    module
}
