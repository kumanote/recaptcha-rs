use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::io;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Codes(HashSet<Code>),

    #[error("API response error: [{status_code:?}]{reason:?}")]
    Gateway { status_code: u16, reason: String },

    #[error("Hyper error: {cause:?}")]
    Hyper { cause: hyper::Error },

    #[error("IO error: {cause:?}")]
    Io { cause: io::Error },

    #[error("Deserialize error: {cause:?}")]
    Deserialize { cause: serde_json::Error },
}

impl From<hyper::Error> for Error {
    fn from(cause: hyper::Error) -> Error {
        Error::Hyper { cause }
    }
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::Io { cause }
    }
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Error {
        Error::Deserialize { cause }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Code {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    BadRequest,
    TimeoutOrDuplicate,
    Unknown(String),
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = String::deserialize(de)?;
        Ok(match &*code {
            "missing-input-secret" => Code::MissingSecret,
            "invalid-input-secret" => Code::InvalidSecret,
            "missing-input-response" => Code::MissingResponse,
            "invalid-input-response" => Code::InvalidResponse,
            "bad-request" => Code::BadRequest,
            "timeout-or-duplicate" => Code::TimeoutOrDuplicate,
            _ => Code::Unknown(code),
        })
    }
}
