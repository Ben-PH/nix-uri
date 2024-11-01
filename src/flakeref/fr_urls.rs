
use std::{fmt::Display, path::Path};

use nom::{
    bytes::complete::{tag, take_until},
    combinator::{opt, rest},
    IResult,
};
use serde::{Deserialize, Serialize};


use crate::{
    error::{NixUriError, NixUriResult},
    parser::{parse_owner_repo_ref, parse_url_type},
};
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UrlType {
    #[default]
    None,
    Https,
    Ssh,
    File,
}

impl TryFrom<&str> for UrlType {
    type Error = NixUriError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use UrlType::*;
        match value {
            "" => Ok(None),
            "https" => Ok(Https),
            "ssh" => Ok(Ssh),
            "file" => Ok(File),
            err => Err(NixUriError::UnknownUrlType(err.into())),
        }
    }
}

impl Display for UrlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlType::None => write!(f, "No Url Type Specified"),
            UrlType::Https => write!(f, "https"),
            UrlType::Ssh => write!(f, "ssh"),
            UrlType::File => write!(f, "file"),
        }
    }
}