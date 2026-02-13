use std::error::Error;
use std::fmt;
use std::fs;


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

pub fn compare_strings(expected: &str, actual: &str) -> Result<(), CompareMismatch> {
    let exp_lines: Vec<&str> = expected.lines().collect();
    let act_lines: Vec<&str> = actual.lines().collect();
    let max_len = exp_lines.len().max(act_lines.len());
    for i in 0..max_len {
        let exp = exp_lines.get(i).copied().unwrap_or("");
        let act = act_lines.get(i).copied().unwrap_or("");
        if exp != act {
            return Err(CompareMismatch {
                line: i + 1,
                expected: exp.to_string(),
                actual: act.to_string(),
            });
        }
    }
    Ok(())
}

pub fn compare_files(expected_path: &str, actual_path: &str) -> Result<(), CompareMismatch> {
    let expected = fs::read_to_string(expected_path).unwrap_or_default();
    let actual = fs::read_to_string(actual_path).unwrap_or_default();
    compare_strings(&expected, &actual)
}
