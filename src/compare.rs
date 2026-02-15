use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};



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
