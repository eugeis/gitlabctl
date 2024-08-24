use gitlab::api::ApiError;
use gitlab::{GitlabError, RestError};
use std::io;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("gitlab error: {}", source)]
    Gitlab {
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
    Serde {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("IoError")]
    Io {
        #[from]
        source: io::Error,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
