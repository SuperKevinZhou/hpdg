use thiserror::Error;

#[derive(Error, Debug)]
pub enum HpdgError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("compare mismatch: {0}")]
    Compare(#[from] crate::compare::CompareMismatch),

    #[error("process error: {0}")]
    Process(String),
}
