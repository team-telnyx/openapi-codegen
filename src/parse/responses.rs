//! Response parsing

use std::collections::HashMap;

use okapi::openapi3::{RefOr, Responses as OkapiResponses};

use super::{Error, ErrorKind, Type};

/// Responses that an API call can return
#[derive(Debug)]
pub struct Responses {
    /// A map of HTTP response code to JSON response object
    ///
    /// Other serialization formats are currently unsupported
    pub responses: HashMap<String, Type>,
}

impl TryFrom<&OkapiResponses> for Responses {
    type Error = Error;

    fn try_from(value: &OkapiResponses) -> Result<Self, Self::Error> {
        // TODO: This is incredibly inflexible
        let responses = value
            .responses
            .iter()
            .map(|(code, response)| match response {
                RefOr::Object(x) => Ok((code, x)),
                RefOr::Ref(_) => Err(Error::from(ErrorKind::Unimplemented)),
            })
            .filter_map(|x| {
                x.map(|(code, x)| {
                    x.content.get("application/json").map(|x| (code, x))
                })
                .transpose()
            })
            .filter_map(|x| {
                x.map(|(code, x)| x.schema.as_ref().map(|x| (code, x)))
                    .transpose()
            })
            .map(|x| x.and_then(|(code, x)| Ok((code, Type::try_from(x)?))))
            .try_fold(HashMap::new(), |mut acc, x| {
                let (code, response) = x?;
                acc.insert(code.clone(), response);
                Ok::<_, Error>(acc)
            })?;

        Ok(Self {
            responses,
        })
    }
}
