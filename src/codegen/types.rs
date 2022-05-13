//! Generate types defined by an OpenAPI spec

use okapi::openapi3::OpenApi;

use crate::parse::Type;

/// Convert an OpenAPI document to its type definitions
pub fn types(openapi: &OpenApi) -> String {
    let components = if let Some(x) = &openapi.components {
        x
    } else {
        todo!()
    };

    let mut code = String::new();

    for (name, object) in &components.schemas {
        let r#type = match Type::try_from(object) {
            Ok(x) => x,
            Err(e) => {
                eprintln!(
                    "error: failed to parse {name}: {}\nbacktrace:\n{:?}",
                    e.kind, e.backtrace
                );

                continue;
            }
        };

        code.push_str("class ");
        code.push_str(name);
        code.push_str(":\n");

        if let Type::Struct(s) = r#type {
            // Struct documentation
            if let Some(docs) = s.docs.as_ref() {
                code.push_str(super::INDENT);
                code.push_str(r#"""""#);
                code.push_str(docs);
                code.push_str(r#"""""#);
                code.push_str("\n\n");
            }

            for (name, data) in &s.fields {
                code.push_str(super::INDENT);
                code.push_str(name);
                code.push_str(": ");
                code.push_str(&field_type(&data.r#type));
                code.push('\n');

                // Field documentation
                if let Some(docs) = data.docs.as_ref() {
                    code.push_str(super::INDENT);
                    code.push_str(r#"""""#);
                    code.push_str(docs);
                    code.push_str(r#"""""#);
                    code.push_str("\n\n");
                }
            }
        } else {
            code.push_str("pass\n");
        }

        code.push_str("\n\n");
    }

    code
}

/// Generate a field's type
fn field_type(ty: &Type) -> String {
    match ty {
        // Simple types
        Type::String => "str".into(),
        Type::Integer => "int".into(),
        Type::Float => "float".into(),
        Type::None => "None".into(),
        Type::Bool => "bool".into(),
        Type::Any => "Any".into(),

        Type::List(ty) => {
            let mut x = "List[".to_owned();
            x.push_str(&field_type(ty));
            x.push(']');

            x
        }

        Type::Set(ty) => {
            let mut x = "Set[".to_owned();
            x.push_str(&field_type(ty));
            x.push(']');

            x
        }

        // I'm pretty sure this means this is a map in all cases, but not
        // *entirely* sure
        Type::Struct(_) => {
            // I still don't really understand the difference betewen `Dict` and
            // `Mapping`. Though, `[str, Any]` is definitely correct because all
            // JSON keys must be strings and the value can be anything.
            "Dict[str, Any]".into()
        }

        // This is a reference to another type
        Type::Ref(x) => {
            // This seems... cursed
            format!(r#""{}""#, x.rsplit('/').next().unwrap().to_owned())
        }
    }
}
