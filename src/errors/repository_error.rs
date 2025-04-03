use oracle;
use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    OracleError(oracle::Error),
    #[allow(dead_code)]
    NotFound,
    CreationFailed,
    UpdateFailed,
    InvalidUuid(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::OracleError(e) => write!(f, "Oracle error: {}", e),
            RepositoryError::NotFound => write!(f, "Record not found"),
            RepositoryError::CreationFailed => write!(f, "Failed to create record"),
            RepositoryError::UpdateFailed => write!(f, "Failed to update record"),
            RepositoryError::InvalidUuid(msg) => write!(f, "Invalid UUID: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<oracle::Error> for RepositoryError {
    fn from(err: oracle::Error) -> Self {
        RepositoryError::OracleError(err)
    }
}
