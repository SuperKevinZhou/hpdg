use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CompareMismatch {
    pub line: usize,
    pub expected: String,
    pub actual: String,
}

impl fmt::Display for CompareMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mismatch at line {}: expected `{}', got `{}'",
            self.line, self.expected, self.actual
        )
    }
}

impl Error for CompareMismatch {}
