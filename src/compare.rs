use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;




#[derive(Debug, Clone)]
/// Describes a mismatch between expected and actual output.
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

/// Custom output grader interface.
pub trait Grader {
    fn grade(&self, expected: &str, actual: &str) -> Result<(), CompareMismatch>;
}

/// Default grader using strict comparison.
pub struct DefaultGrader;

impl Grader for DefaultGrader {
    fn grade(&self, expected: &str, actual: &str) -> Result<(), CompareMismatch> {
        compare_strings(expected, actual)
    }
}

/// Grader that ignores whitespace differences.
pub struct WhitespaceInsensitiveGrader;

impl Grader for WhitespaceInsensitiveGrader {
    fn grade(&self, expected: &str, actual: &str) -> Result<(), CompareMismatch> {
        compare_strings_normalized(expected, actual)
    }
}

pub fn compare_with_grader<G: Grader>(
    expected: &str,
    actual: &str,
    grader: &G,
) -> Result<(), CompareMismatch> {
    grader.grade(expected, actual)
}

/// Compare two strings line by line.
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

/// Compare two files line by line.
pub fn compare_files(expected_path: &str, actual_path: &str) -> Result<(), CompareMismatch> {
    let expected = fs::read_to_string(expected_path).unwrap_or_default();
    let actual = fs::read_to_string(actual_path).unwrap_or_default();
    compare_strings(&expected, &actual)
}

fn run_program(cmd: &[&str], input: &str) -> Result<String, String> {
    if cmd.is_empty() {
        return Err("empty command".to_string());
    }
    let mut command = Command::new(cmd[0]);
    if cmd.len() > 1 {
        command.args(&cmd[1..]);
    }
    let mut child = command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
        .map_err(|e| e.to_string())?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes()).map_err(|e| e.to_string())?;
    }
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Run two programs and compare their outputs.
pub fn compare_programs(
    expected_cmd: &[&str],
    actual_cmd: &[&str],
    input: &str,
) -> Result<(), CompareMismatch> {
    let expected_out = run_program(expected_cmd, input)
        .unwrap_or_else(|e| format!("<<error>> {}", e));
    let actual_out = run_program(actual_cmd, input)
        .unwrap_or_else(|e| format!("<<error>> {}", e));
    compare_strings(&expected_out, &actual_out)
}

/// Run two programs and compare outputs with a custom grader.
pub fn compare_programs_with_grader<G: Grader>(
    expected_cmd: &[&str],
    actual_cmd: &[&str],
    input: &str,
    grader: &G,
) -> Result<(), CompareMismatch> {
    let expected_out = run_program(expected_cmd, input)
        .unwrap_or_else(|e| format!("<<error>> {}", e));
    let actual_out = run_program(actual_cmd, input)
        .unwrap_or_else(|e| format!("<<error>> {}", e));
    grader.grade(&expected_out, &actual_out)
}

/// Compare multiple string pairs in parallel.
pub fn compare_strings_parallel(pairs: &[(String, String)], threads: usize) -> Result<(), CompareMismatch> {
    if pairs.is_empty() {
        return Ok(());
    }
    let worker_count = threads.max(1);
    let chunk_size = (pairs.len() + worker_count - 1) / worker_count;
    let (tx, rx) = mpsc::channel();
    for chunk in pairs.chunks(chunk_size) {
        let tx = tx.clone();
        let local = chunk.to_vec();
        thread::spawn(move || {
            for (expected, actual) in local {
                if let Err(err) = compare_strings(&expected, &actual) {
                    let _ = tx.send(Err(err));
                    return;
                }
            }
            let _ = tx.send(Ok(()));
        });
    }
    drop(tx);
    for msg in rx {
        if let Err(err) = msg {
            return Err(err);
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_strings() {
        assert!(compare_strings("a\nb", "a\nb").is_ok());
        assert!(compare_strings("a", "b").is_err());
    }

    #[test]
    fn test_compare_strings_normalized() {
        assert!(compare_strings_normalized("a  b", "a b").is_ok());
    }

    #[test]
    fn test_compare_strings_parallel() {
        let pairs = vec![("ok".to_string(), "ok".to_string())];
        assert!(compare_strings_parallel(&pairs, 2).is_ok());
    }

    #[test]
    fn test_custom_grader() {
        let grader = WhitespaceInsensitiveGrader;
        assert!(compare_with_grader("a  b", "a b", &grader).is_ok());
    }
}
