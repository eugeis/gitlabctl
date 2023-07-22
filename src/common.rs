use std::io;
use gitlab::api::{ApiError};
use gitlab::{GitlabError, RestError};
use thiserror;
use serde_yaml;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("gitlab error: {}", source)]
    GitlabError {
        #[from]
        source: GitlabError
    },
    #[error("general error: {}", source)]
    General {
        #[from]
        source: ApiError<RestError>,
    },
    #[error("AlreadyHandled")]
    AlreadyHandled,

    #[error("SerdeError")]
    SerdeError {
        #[from]
        source: serde_yaml::Error
    },

    #[error("IoError")]
    IoError {
        #[from]
        source: io::Error
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
