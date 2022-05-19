//! OpenAPI security scheme parsing

use std::collections::BTreeMap;

use okapi::{
    openapi3::{
        RefOr, SecurityScheme as OkapiSecurityScheme, SecuritySchemeData,
    },
    schemars::Map,
};

/// HTTP authentication methods
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SecurityScheme {
    /// HTTP basic auth
    BasicAuth,
}

/// Parse OpenAPI security schemes
// It's only zero-sized for now
#[allow(clippy::zero_sized_map_values)]
pub fn security_schemes(
    security_schemes: &Map<String, RefOr<OkapiSecurityScheme>>,
) -> BTreeMap<String, SecurityScheme> {
    let mut schemes = BTreeMap::default();

    for (name, scheme) in security_schemes {
        let object = if let RefOr::Object(x) = scheme {
            x
        } else {
            eprintln!("unsupported security scheme: {name}");
            continue;
        };

        let http = if let SecuritySchemeData::Http {
            scheme,
            bearer_format: None,
        } = &object.data
        {
            scheme
        } else {
            eprintln!("unsupported security scheme: {name}");
            continue;
        };

        if http.contains("basic") {
            schemes.insert(name.clone(), SecurityScheme::BasicAuth);
        } else {
            eprintln!("unsupported security scheme: {name}");
            continue;
        }
    }

    schemes
}
