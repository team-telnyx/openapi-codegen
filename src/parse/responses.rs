/// Response parsing
use okapi::openapi3::{RefOr, Responses as OkapiResponses};

use super::{Error, ErrorKind, Type};

/// Responses that an API call can return
#[derive(Debug)]
pub struct Responses {
    /// A successful response
    pub good: Type,
}

impl TryFrom<&OkapiResponses> for Responses {
    type Error = Error;

    fn try_from(value: &OkapiResponses) -> Result<Self, Self::Error> {
        // TODO: This is incredibly inflexible
        let good = value
            .responses
            .get("200")
            .iter()
            .map(|x| match x {
                RefOr::Object(x) => Ok(x),
                RefOr::Ref(_) => Err(ErrorKind::Unimplemented.into()),
            })
            .filter_map(|x| {
                x.map(|x| x.content.get("application/json")).transpose()
            })
            .filter_map(|x| x.map(|x| dbg!(x.schema.as_ref())).transpose())
            .map(|x| x.and_then(TryFrom::try_from))
            .next()
            .unwrap_or(Ok(Type::None))?;

        Ok(Self {
            good: dbg!(good),
        })
    }
}
