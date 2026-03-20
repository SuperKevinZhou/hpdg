//! Shared error types for fallible `hpdg` workflows.

use thiserror::Error;

/// Error type used by higher-level crate APIs.
#[derive(Error, Debug)]
pub enum HpdgError {
    /// Wrapper around I/O failures.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Wrapper around output-comparison mismatches.
    #[error("compare mismatch: {0}")]
    Compare(#[from] crate::compare::CompareMismatch),

    /// Generic process-execution failure with a human-readable message.
    #[error("process error: {0}")]
    Process(String),
}
