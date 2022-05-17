//! Where to begin to converting OpenAPI to source code

use std::error::Error as StdError;

use okapi::openapi3::OpenApi;

/// Convert YAML into a string containing source code
///
/// # Errors
///
/// This function will fail if `s` is not a valid YAML-formatted OpenAPI
/// document.
pub fn from_yaml(s: &str) -> Result<String, Box<dyn StdError>> {
    let t = serde_yaml::from_str(s)?;

    Ok(from_openapi(&t))
}

/// Convert JSON into a string containing source code
///
/// # Errors
///
/// This function will fail if `s` is not a valid JSON-formatted OpenAPI
/// document.
pub fn from_json(s: &str) -> Result<String, Box<dyn StdError>> {
    let t = serde_json::from_str(s)?;

    Ok(from_openapi(&t))
}

/// Converts an OpenAPI specification into a string containing source code
#[must_use = "It's pointless to call this function unless you use the result"]
pub fn from_openapi(openapi: &OpenApi) -> String {
    crate::codegen::module(openapi)
}
