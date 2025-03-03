
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    InvalidModelName(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidModelName(model) => {
                write!(f, "Invalid model name: {}, available models are: {}", model, crate::client::SUPPORTED_MODELS.join(", "))
            }
        }
    }
}

use reqwest::Error as ReqwestError;

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error {
            kind: ErrorKind::InvalidModelName(error.to_string()),
        }
    }
}