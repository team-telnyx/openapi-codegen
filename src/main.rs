#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::as_conversions)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::negative_feature_names)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_feature_names)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_add)]
#![warn(clippy::string_slice)]
#![warn(clippy::string_to_string)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::wildcard_dependencies)]

use std::{
    error::Error as StdError,
    io::{BufReader, Read, Write},
};

use clap::StructOpt;
use okapi::{
    openapi3::{Operation, ParameterValue, RefOr},
    schemars::schema::{InstanceType, SingleOrVec},
};

mod args;
mod codegen;
mod entrypoint;
mod parse;

fn main() -> Result<(), Box<dyn StdError>> {
    let args = args::Args::parse();

    // Allocate 100KiB to start, these files are usually pretty large
    let mut s = String::with_capacity(100 * 1024);

    BufReader::new(std::io::stdin()).read_to_string(&mut s)?;

    let code = match args {
        args::Args::Yaml => entrypoint::from_yaml(&s)?,
        args::Args::Json => entrypoint::from_json(&s)?,
    };

    print!("{code}");

    // TTYs are line buffered, so we have to manually flush after potentially
    // not ending with a newline
    std::io::stdout().flush()?;

    Ok(())
}

/// Resolves the OpenAPI type for a given parameter name
fn resolve_type_for(parameter_name: &str, operation: &Operation) -> String {
    let parameter = operation
        .parameters
        .iter()
        .find_map(|parameter| {
            let parameter = if let RefOr::Object(x) = parameter {
                x
            } else {
                return None;
            };

            (parameter.name == parameter_name).then(|| parameter)
        })
        .expect("couldn't find requested parameter");

    let schema = match &parameter.value {
        ParameterValue::Schema {
            schema,
            ..
        } => schema,

        ParameterValue::Content {
            ..
        } => todo!(),
    };

    let ty = match &schema.instance_type {
        Some(SingleOrVec::Single(x)) => match x.as_ref() {
            InstanceType::Number => "float",
            InstanceType::Integer => "int",
            InstanceType::Boolean => "bool",
            InstanceType::String => "str",
            InstanceType::Null => "None",

            InstanceType::Object | InstanceType::Array => todo!(),
        },

        Some(SingleOrVec::Vec(_)) | None => todo!(),
    };

    if parameter.required {
        ty.to_owned()
    } else {
        format!("Optional[{ty}]")
    }
}
