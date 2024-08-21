use gitlab::api::ApiError;
use gitlab::{GitlabError, RestError};
use serde_yaml;
use std::io;
use thiserror;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("gitlab error: {}", source)]
    GitlabError {
        #[from]
        source: GitlabError,
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
        source: serde_yaml::Error,
    },

    #[error("IoError")]
    IoError {
        #[from]
        source: io::Error,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
