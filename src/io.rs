//! Testcase I/O buffers, file naming, process execution, and streaming output.
//!
//! This module is the glue layer for many `hpdg` workflows. You can build input/output
//! buffers in memory, flush them to disk, invoke a standard solution, capture its output,
//! and scale up to batches or streaming writes.
//!
//! # Example
//!
//! ```rust
//! use hpdg::io::IO;
//!
//! let mut io = IO::new("sample".to_string());
//! io.input_writeln("1 2 3");
//! io.output_writeln("ok");
//! assert!(io.last_capture().is_none());
//! ```
//!
//! ## Naming Model
//!
//! `IO` keeps separate input/output filenames and can derive them from a shared prefix plus
//! optional testcase ids.
//!
//! ```text
//! prefix = "cases/data"
//! data_id = 3
//! data_id_separator = "_"
//! data_id_width = Some(2)
//!
//! => input file  = "cases/data_03.in"
//! => output file = "cases/data_03.out"
//! ```
//!
//! ## Output Shape Examples
//!
//! ```text
//! io.input_writeln_slice(&[1, 2, 3], " ")
//! => "1 2 3\n"
//!
//! io.input_writeln_matrix(&[vec![1, 2], vec![3, 4]], " ")
//! => "1 2\n3 4\n"
//! ```

/// Formatting strategy used by [`IO`] write helpers.
pub trait Formatter {
    fn format_item(&self, item: &dyn std::fmt::Display) -> String;
    fn join(&self, items: &[String]) -> String;

    /// Format and join an iterator of displayable items.
    fn format_iter<I, T>(&self, items: I) -> String
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
        Self: Sized,
    {
        let rendered: Vec<String> = items
            .into_iter()
            .map(|item| self.format_item(&item))
            .collect();
        self.join(&rendered)
    }
}

#[derive(Debug, Clone)]
/// A formatter that joins items with a separator.
///
/// ```rust
/// use hpdg::io::{Formatter, SepFormatter};
///
/// let fmt = SepFormatter::new(", ".to_string());
/// assert_eq!(fmt.format_iter([1, 2, 3]), "1, 2, 3");
/// ```
pub struct SepFormatter {
    sep: String,
}

impl SepFormatter {
    /// Create a formatter that joins items with `sep`.
    pub fn new(sep: String) -> Self {
        Self { sep }
    }
}

impl Formatter for SepFormatter {
    fn format_item(&self, item: &dyn std::fmt::Display) -> String {
        format!("{}", item)
    }

    fn join(&self, items: &[String]) -> String {
        items.join(&self.sep)
    }
}

impl Default for SepFormatter {
    fn default() -> Self {
        Self {
            sep: " ".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
/// Capture of a program execution.
pub struct OutputCapture {
    /// Process exit code, if available.
    pub code: Option<i32>,
    /// Whether the process exited successfully.
    pub success: bool,
    /// Raw stdout bytes.
    pub stdout: Vec<u8>,
    /// Raw stderr bytes.
    pub stderr: Vec<u8>,
    /// Decoded stdout text.
    pub stdout_text: String,
    /// Decoded stderr text.
    pub stderr_text: String,
}

#[derive(Debug)]
/// I/O and process-related errors for the [`io`](crate::io) module.
pub enum IOError {
    Io(std::io::Error),
    Process(String),
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOError::Io(err) => write!(f, "io error: {}", err),
            IOError::Process(msg) => write!(f, "process error: {}", msg),
        }
    }
}

impl std::error::Error for IOError {}

impl From<std::io::Error> for IOError {
    fn from(value: std::io::Error) -> Self {
        IOError::Io(value)
    }
}

/// Result type used by higher-level I/O helpers.
pub type IOResult<T> = Result<T, IOError>;

#[derive(Debug, Clone)]
/// Testcase input/output buffer and file naming helper.
///
/// ```no_run
/// use hpdg::io::IO;
///
/// let mut io = IO::new("cases/data".to_string());
/// io.data_id_separator("_".to_string())
///     .data_id_width(Some(2))
///     .data_id(3)
///     .input_writeln_slice(&[1, 2, 3], " ")
///     .output_writeln("6");
/// let _ = io.flush_to_disk();
/// ```
pub struct IO {
    input_file: String,
    output_file: String,
    file_prefix: String,
    input_prefix: Option<String>,
    output_prefix: Option<String>,
    data_id: Option<usize>,
    data_id_separator: String,
    data_id_width: Option<usize>,
    input_suffix: String,
    output_suffix: String,
    auto_create_dirs: bool,
    auto_clean_files: bool,
    allow_overwrite: bool,

    input_content: String,
    output_content: String,
    input_bytes: Vec<u8>,
    output_bytes: Vec<u8>,
    last_stderr: Vec<u8>,
    last_stderr_text: String,
    last_capture: Option<OutputCapture>,
    logger: Option<fn(&str)>,
}

impl IO {
    /// Create a new testcase helper with a file prefix such as `"sample"` or `"data/1"`.
    pub fn new(file_prefix: String) -> IO {
        let input_suffix = "in".to_string();
        let output_suffix = "out".to_string();
        let input_file = format!("{}.{}", file_prefix, input_suffix);
        let output_file = format!("{}.{}", file_prefix, output_suffix);

        IO {
            input_file,
            output_file,
            file_prefix,
            input_prefix: None,
            output_prefix: None,
            data_id: None,
            data_id_separator: "".to_string(),
            data_id_width: None,
            input_suffix,
            output_suffix,
            auto_create_dirs: true,
            auto_clean_files: false,
            allow_overwrite: false,
            input_content: String::new(),
            output_content: String::new(),
            input_bytes: Vec::new(),
            output_bytes: Vec::new(),
            last_stderr: Vec::new(),
            last_stderr_text: String::new(),
            last_capture: None,
            logger: None,
        }
    }

    /// Override the full input file path.
    pub fn input_file(&mut self, input_file: String) -> &mut Self {
        self.input_file = Self::normalize_path(&input_file);
        self
    }

    /// Override the full output file path.
    pub fn output_file(&mut self, output_file: String) -> &mut Self {
        self.output_file = Self::normalize_path(&output_file);
        self
    }

    /// Set the base file prefix and rebuild derived filenames.
    pub fn file_prefix(&mut self, file_prefix: String) -> &mut Self {
        self.file_prefix = file_prefix;
        self.rebuild_filenames();
        self
    }

    /// Add a prefix in front of the input filename.
    pub fn input_prefix(&mut self, input_prefix: String) -> &mut Self {
        self.input_prefix = Some(input_prefix);
        self.rebuild_filenames();
        self
    }

    /// Add a prefix in front of the output filename.
    pub fn output_prefix(&mut self, output_prefix: String) -> &mut Self {
        self.output_prefix = Some(output_prefix);
        self.rebuild_filenames();
        self
    }

    /// Remove any custom input filename prefix.
    pub fn clear_input_prefix(&mut self) -> &mut Self {
        self.input_prefix = None;
        self.rebuild_filenames();
        self
    }

    /// Remove any custom output filename prefix.
    pub fn clear_output_prefix(&mut self) -> &mut Self {
        self.output_prefix = None;
        self.rebuild_filenames();
        self
    }

    /// Attach a numeric testcase id to derived filenames.
    pub fn data_id(&mut self, data_id: usize) -> &mut Self {
        self.data_id = Some(data_id);
        self.rebuild_filenames();
        self
    }

    /// Set the separator placed before the testcase id.
    pub fn data_id_separator(&mut self, separator: String) -> &mut Self {
        self.data_id_separator = separator;
        self.rebuild_filenames();
        self
    }

    /// Set zero-padding width for the testcase id.
    pub fn data_id_width(&mut self, width: Option<usize>) -> &mut Self {
        self.data_id_width = width;
        self.rebuild_filenames();
        self
    }

    /// Remove the testcase id from derived filenames.
    pub fn clear_data_id(&mut self) -> &mut Self {
        self.data_id = None;
        self.rebuild_filenames();
        self
    }

    /// Change the logical input suffix used when rebuilding filenames.
    pub fn input_suffix(&mut self, input_suffix: String) -> &mut Self {
        self.input_suffix = input_suffix.clone();
        self.rebuild_filenames();
        self
    }

    /// Change the logical output suffix used when rebuilding filenames.
    pub fn output_suffix(&mut self, output_suffix: String) -> &mut Self {
        self.output_suffix = output_suffix.clone();
        self.rebuild_filenames();
        self
    }

    /// Change the file extension used by the input file.
    pub fn input_extension(&mut self, input_extension: String) -> &mut Self {
        self.input_suffix = input_extension;
        self.rebuild_filenames();
        self
    }

    /// Change the file extension used by the output file.
    pub fn output_extension(&mut self, output_extension: String) -> &mut Self {
        self.output_suffix = output_extension;
        self.rebuild_filenames();
        self
    }

    /// Enable or disable automatic directory creation before writing files.
    pub fn auto_create_dirs(&mut self, enabled: bool) -> &mut Self {
        self.auto_create_dirs = enabled;
        self
    }

    /// Enable or disable cleanup of existing files before writing.
    pub fn auto_clean_files(&mut self, enabled: bool) -> &mut Self {
        self.auto_clean_files = enabled;
        self
    }

    /// Install a lightweight text logger callback.
    pub fn logger(&mut self, logger: Option<fn(&str)>) -> &mut Self {
        self.logger = logger;
        self
    }

    /// Allow or forbid overwriting already existing target files.
    pub fn allow_overwrite(&mut self, enabled: bool) -> &mut Self {
        self.allow_overwrite = enabled;
        self
    }

    fn rebuild_filenames(&mut self) {
        let input_prefix = self.input_prefix.as_deref().unwrap_or(&self.file_prefix);
        let output_prefix = self.output_prefix.as_deref().unwrap_or(&self.file_prefix);

        if let Some(data_id) = self.data_id {
            let id = if let Some(width) = self.data_id_width {
                format!("{:0width$}", data_id, width = width)
            } else {
                data_id.to_string()
            };
            let joiner = &self.data_id_separator;
            self.input_file = Self::normalize_path(&format!(
                "{}{}{}.{}",
                input_prefix, joiner, id, self.input_suffix
            ));
            self.output_file = Self::normalize_path(&format!(
                "{}{}{}.{}",
                output_prefix, joiner, id, self.output_suffix
            ));
        } else {
            self.input_file =
                Self::normalize_path(&format!("{}.{}", input_prefix, self.input_suffix));
            self.output_file =
                Self::normalize_path(&format!("{}.{}", output_prefix, self.output_suffix));
        }
    }

    fn normalize_path(path: &str) -> String {
        let sep = std::path::MAIN_SEPARATOR.to_string();
        let mut buf = path.trim().to_string();
        buf = buf.replace(['/', '\\'], &sep);
        buf
    }
}

impl IO {
    /// Write a value to the input buffer without a trailing newline.
    pub fn input_write<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.input_content, format_args!("{}", s));
        self
    }

    /// Write a value to the output buffer without a trailing newline.
    pub fn output_write<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.output_content, format_args!("{}", s));
        self
    }

    /// Write a value to the input buffer followed by `\n`.
    pub fn input_writeln<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.input_content, format_args!("{}\n", s));
        self
    }

    /// Write a value to the output buffer followed by `\n`.
    pub fn output_writeln<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.output_content, format_args!("{}\n", s));
        self
    }

    /// Write an iterator into the input buffer joined by `sep`.
    pub fn input_write_sep<I, T>(&mut self, items: I, sep: &str) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        let mut iter = items.into_iter();
        if let Some(first) = iter.next() {
            let _ = std::fmt::Write::write_fmt(&mut self.input_content, format_args!("{}", first));
        }
        for item in iter {
            let _ = std::fmt::Write::write_fmt(
                &mut self.input_content,
                format_args!("{}{}", sep, item),
            );
        }
        self
    }

    /// Write an iterator into the output buffer joined by `sep`.
    pub fn output_write_sep<I, T>(&mut self, items: I, sep: &str) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        let mut iter = items.into_iter();
        if let Some(first) = iter.next() {
            let _ = std::fmt::Write::write_fmt(&mut self.output_content, format_args!("{}", first));
        }
        for item in iter {
            let _ = std::fmt::Write::write_fmt(
                &mut self.output_content,
                format_args!("{}{}", sep, item),
            );
        }
        self
    }

    /// Write a separator-joined iterator to the input buffer and append `\n`.
    pub fn input_writeln_sep<I, T>(&mut self, items: I, sep: &str) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        self.input_write_sep(items, sep);
        let _ = std::fmt::Write::write_str(&mut self.input_content, "\n");
        self
    }

    /// Write a separator-joined iterator to the output buffer and append `\n`.
    pub fn output_writeln_sep<I, T>(&mut self, items: I, sep: &str) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        self.output_write_sep(items, sep);
        let _ = std::fmt::Write::write_str(&mut self.output_content, "\n");
        self
    }

    /// Format an iterator with a custom [`Formatter`] and write it to the input buffer.
    pub fn input_write_with<I, T>(&mut self, formatter: &dyn Formatter, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        let rendered: Vec<String> = items
            .into_iter()
            .map(|item| formatter.format_item(&item))
            .collect();
        self.input_content.push_str(&formatter.join(&rendered));
        self
    }

    /// Format an iterator with a custom [`Formatter`] and write it to the output buffer.
    pub fn output_write_with<I, T>(&mut self, formatter: &dyn Formatter, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        let rendered: Vec<String> = items
            .into_iter()
            .map(|item| formatter.format_item(&item))
            .collect();
        self.output_content.push_str(&formatter.join(&rendered));
        self
    }

    /// Write a slice to the input buffer as one line separated by `sep`.
    ///
    /// Conceptually this produces output like `1 2 3\n` when `sep == " "`.
    pub fn input_writeln_slice<T: std::fmt::Display>(
        &mut self,
        slice: &[T],
        sep: &str,
    ) -> &mut Self {
        self.input_writeln_sep(slice.iter(), sep)
    }

    /// Write a slice to the output buffer as one line separated by `sep`.
    ///
    /// Conceptually this produces output like `1 2 3\n` when `sep == " "`.
    pub fn output_writeln_slice<T: std::fmt::Display>(
        &mut self,
        slice: &[T],
        sep: &str,
    ) -> &mut Self {
        self.output_writeln_sep(slice.iter(), sep)
    }

    /// Write a matrix to the input buffer, one row per line.
    ///
    /// For example, `[[1, 2], [3, 4]]` with `sep == " "` becomes:
    ///
    /// ```text
    /// 1 2
    /// 3 4
    /// ```
    pub fn input_writeln_matrix<T: std::fmt::Display>(
        &mut self,
        matrix: &[Vec<T>],
        sep: &str,
    ) -> &mut Self {
        for row in matrix {
            self.input_writeln_sep(row.iter(), sep);
        }
        self
    }

    /// Write a matrix to the output buffer, one row per line.
    ///
    /// For example, `[[1, 2], [3, 4]]` with `sep == " "` becomes:
    ///
    /// ```text
    /// 1 2
    /// 3 4
    /// ```
    pub fn output_writeln_matrix<T: std::fmt::Display>(
        &mut self,
        matrix: &[Vec<T>],
        sep: &str,
    ) -> &mut Self {
        for row in matrix {
            self.output_writeln_sep(row.iter(), sep);
        }
        self
    }

    /// Clear both textual and binary input buffers.
    pub fn input_clear(&mut self) -> &mut Self {
        self.input_content.clear();
        self.input_bytes.clear();
        self
    }

    /// Clear both textual and binary output buffers.
    pub fn output_clear(&mut self) -> &mut Self {
        self.output_content.clear();
        self.output_bytes.clear();
        self
    }

    /// Append raw bytes to the input byte buffer.
    pub fn input_write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.input_bytes.extend_from_slice(bytes);
        self
    }

    /// Append raw bytes to the output byte buffer.
    pub fn output_write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.output_bytes.extend_from_slice(bytes);
        self
    }

    /// Flush the textual input buffer to the configured input file.
    pub fn flush_input_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.input_file)?;
        std::fs::write(&self.input_file, &self.input_content)
    }

    /// Flush the textual output buffer to the configured output file.
    pub fn flush_output_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.output_file)?;
        std::fs::write(&self.output_file, &self.output_content)
    }

    /// Flush the binary input buffer to the configured input file.
    pub fn flush_input_bytes_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.input_file)?;
        std::fs::write(&self.input_file, &self.input_bytes)
    }

    /// Flush the binary output buffer to the configured output file.
    pub fn flush_output_bytes_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.output_file)?;
        std::fs::write(&self.output_file, &self.output_bytes)
    }

    /// Flush both binary buffers to disk.
    pub fn flush_bytes_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.flush_input_bytes_to_disk()?;
        self.flush_output_bytes_to_disk()?;
        Ok(())
    }

    /// Flush both text buffers and byte buffers to their configured files.
    ///
    /// ```no_run
    /// use hpdg::io::IO;
    ///
    /// let mut io = IO::new("sample".to_string());
    /// io.input_writeln("1 2 3");
    /// io.output_writeln("6");
    /// let _ = io.flush_to_disk();
    /// ```
    pub fn flush_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.log("flush_to_disk: start");
        self.flush_input_to_disk()?;
        self.flush_output_to_disk()?;
        self.log("flush_to_disk: done");
        Ok(())
    }

    /// Fallible wrapper around [`IO::flush_to_disk`] using [`IOError`].
    pub fn flush_to_disk_result(&self) -> IOResult<()> {
        self.flush_to_disk().map_err(IOError::from)
    }

    fn ensure_no_conflict(&self) -> std::io::Result<()> {
        if self.input_file == self.output_file {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "input and output file paths conflict",
            ));
        }
        Ok(())
    }

    /// Return information captured from the most recent child-process execution.
    pub fn last_capture(&self) -> Option<&OutputCapture> {
        self.last_capture.as_ref()
    }

    fn set_capture(&mut self, status: &std::process::ExitStatus, stdout: Vec<u8>, stderr: Vec<u8>) {
        self.output_bytes = stdout.clone();
        self.output_content = String::from_utf8_lossy(&stdout).to_string();
        self.last_stderr = stderr.clone();
        self.last_stderr_text = String::from_utf8_lossy(&stderr).to_string();
        self.last_capture = Some(OutputCapture {
            code: status.code(),
            success: status.success(),
            stdout: stdout.clone(),
            stderr: stderr.clone(),
            stdout_text: String::from_utf8_lossy(&stdout).to_string(),
            stderr_text: String::from_utf8_lossy(&stderr).to_string(),
        });
    }

    fn log(&self, msg: &str) {
        if let Some(logger) = self.logger {
            logger(msg);
        }
    }

    fn ensure_exit_status(&self, status: &std::process::ExitStatus) -> std::io::Result<()> {
        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("process exited with status: {status}"),
            ))
        }
    }

    fn wait_with_timeout(
        child: &mut std::process::Child,
        timeout: std::time::Duration,
    ) -> std::io::Result<std::process::ExitStatus> {
        let start = std::time::Instant::now();
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok(status);
            }
            if start.elapsed() >= timeout {
                let _ = Self::kill_child(child);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "process timed out",
                ));
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn kill_child(child: &mut std::process::Child) -> std::io::Result<()> {
        let _ = child.kill();
        let _ = child.wait();
        Ok(())
    }

    /// Remove the currently configured input and output files if they exist.
    pub fn cleanup_files(&self) -> std::io::Result<()> {
        let _ = std::fs::remove_file(&self.input_file);
        let _ = std::fs::remove_file(&self.output_file);
        Ok(())
    }

    /// Open a streaming writer for the configured input file.
    ///
    /// This is useful when the full testcase would be expensive to buffer in memory first.
    pub fn open_input_stream(&self) -> std::io::Result<IOStream> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.input_file)?;
        let file = std::fs::File::create(&self.input_file)?;
        Ok(IOStream {
            writer: std::io::BufWriter::new(file),
        })
    }

    /// Open a streaming writer for the configured output file.
    ///
    /// This is useful when the full testcase would be expensive to buffer in memory first.
    pub fn open_output_stream(&self) -> std::io::Result<IOStream> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.output_file)?;
        let file = std::fs::File::create(&self.output_file)?;
        Ok(IOStream {
            writer: std::io::BufWriter::new(file),
        })
    }

    #[cfg(feature = "proc")]
    /// Run an external program and store its stdout in the output buffer.
    pub fn output_gen(&mut self, program: &str) -> std::io::Result<()> {
        self.log("output_gen: start");
        let mut child = std::process::Command::new(program)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(self.input_content.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        self.ensure_exit_status(&output.status)?;
        self.set_capture(&output.status, output.stdout, output.stderr);
        self.log("output_gen: done");
        Ok(())
    }

    #[cfg(feature = "proc")]
    /// Fallible wrapper around [`IO::output_gen`] using [`IOError`].
    pub fn output_gen_result(&mut self, program: &str) -> IOResult<()> {
        self.output_gen(program).map_err(IOError::from)
    }

    #[cfg(feature = "proc")]
    /// Run a program with the current input buffer and return its stdout as a `String`.
    pub fn output_gen_string_only(&self, program: &str) -> std::io::Result<String> {
        let mut child = std::process::Command::new(program)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(self.input_content.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        self.ensure_exit_status(&output.status)?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[cfg(feature = "proc")]
    /// Run an external program using the generated input/output files directly.
    ///
    /// This mirrors a common OI workflow where the standard solution reads from the input file
    /// and writes to the output file.
    pub fn output_gen_with_files(&mut self, program: &str) -> std::io::Result<()> {
        self.log("output_gen_with_files: start");
        self.flush_input_to_disk()?;
        let input_file = std::fs::File::open(&self.input_file)?;
        let output_file = std::fs::File::create(&self.output_file)?;

        let mut child = std::process::Command::new(program)
            .stdin(input_file)
            .stdout(output_file)
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let mut stderr = child.stderr.take();
        let status = child.wait()?;

        let stdout = std::fs::read(&self.output_file)?;
        let stderr = if let Some(mut stderr) = stderr.take() {
            use std::io::Read;
            let mut buf = Vec::new();
            let _ = stderr.read_to_end(&mut buf);
            buf
        } else {
            Vec::new()
        };
        self.set_capture(&status, stdout, stderr);
        self.ensure_exit_status(&status)?;
        self.log("output_gen_with_files: done");
        Ok(())
    }

    #[cfg(feature = "proc")]
    /// Run an external program and enforce a timeout in milliseconds.
    pub fn output_gen_with_timeout(
        &mut self,
        program: &str,
        timeout: std::time::Duration,
    ) -> std::io::Result<()> {
        self.output_gen_with_files_timeout(program, timeout)
    }

    #[cfg(feature = "proc")]
    /// Run an external program with file-based I/O and enforce a timeout.
    pub fn output_gen_with_files_timeout(
        &mut self,
        program: &str,
        timeout: std::time::Duration,
    ) -> std::io::Result<()> {
        self.log("output_gen_with_files_timeout: start");
        self.flush_input_to_disk()?;
        let input_file = std::fs::File::open(&self.input_file)?;
        let output_file = std::fs::File::create(&self.output_file)?;

        let mut child = std::process::Command::new(program)
            .stdin(input_file)
            .stdout(output_file)
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let mut stderr = child.stderr.take();
        let status = Self::wait_with_timeout(&mut child, timeout)?;
        let stdout = std::fs::read(&self.output_file)?;
        let stderr = if let Some(mut stderr) = stderr.take() {
            use std::io::Read;
            let mut buf = Vec::new();
            let _ = stderr.read_to_end(&mut buf);
            buf
        } else {
            Vec::new()
        };
        self.set_capture(&status, stdout, stderr);
        self.ensure_exit_status(&status)?;
        self.log("output_gen_with_files_timeout: done");
        Ok(())
    }

    #[cfg(all(feature = "parallel", feature = "proc"))]
    /// Run the same external program for multiple [`IO`] instances in parallel.
    ///
    /// This is convenient when a batch of independently generated testcases should all be
    /// post-processed by the same standard solution.
    pub fn output_gen_parallel(ios: &mut [IO], program: &str) -> std::io::Result<()> {
        let program = program.to_string();
        let mut first_err: Option<std::io::Error> = None;

        std::thread::scope(|s| {
            let mut handles = Vec::with_capacity(ios.len());
            for io in ios {
                let program = program.clone();
                handles.push(s.spawn(move || io.output_gen(&program)));
            }

            for handle in handles {
                match handle.join() {
                    Ok(Ok(())) => {}
                    Ok(Err(err)) => {
                        if first_err.is_none() {
                            first_err = Some(err);
                        }
                    }
                    Err(_) => {
                        if first_err.is_none() {
                            first_err = Some(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "worker thread panicked",
                            ));
                        }
                    }
                }
            }
        });

        if let Some(err) = first_err {
            Err(err)
        } else {
            Ok(())
        }
    }

    fn prepare_path(&self, path: &str) -> std::io::Result<()> {
        if self.auto_create_dirs {
            if let Some(parent) = std::path::Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)?;
                }
            }
        }
        if self.auto_clean_files {
            let _ = std::fs::remove_file(path);
        }
        if !self.allow_overwrite && std::path::Path::new(path).exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "target file already exists",
            ));
        }
        Ok(())
    }
}

/// Streaming writer interface for incremental output.
pub trait StreamingWriter {
    /// Write a value without a trailing newline.
    fn write_item<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()>;
    /// Write a value followed by `\n`.
    fn writeln_item<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()>;
    /// Write an iterator joined by `sep`.
    fn write_sep<I, T>(&mut self, items: I, sep: &str) -> std::io::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display;
    /// Flush buffered output.
    fn flush(&mut self) -> std::io::Result<()>;
}

/// A streaming writer to avoid buffering the whole output in memory.
///
/// Use this when you want the ergonomics of `write`/`writeln` but prefer incremental output.
pub struct IOStream {
    writer: std::io::BufWriter<std::fs::File>,
}

impl IOStream {
    /// Write a value without a trailing newline.
    pub fn write<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()> {
        use std::io::Write;
        write!(self.writer, "{}", s)
    }

    /// Write a value followed by `\n`.
    pub fn writeln<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(self.writer, "{}", s)
    }

    /// Write an iterator joined by `sep`.
    pub fn write_sep<I, T>(&mut self, items: I, sep: &str) -> std::io::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        use std::io::Write;
        let mut iter = items.into_iter();
        if let Some(first) = iter.next() {
            write!(self.writer, "{}", first)?;
        }
        for item in iter {
            write!(self.writer, "{}{}", sep, item)?;
        }
        Ok(())
    }

    /// Flush buffered output.
    pub fn flush(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        self.writer.flush()
    }
}

impl StreamingWriter for IOStream {
    fn write_item<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()> {
        self.write(s)
    }

    fn writeln_item<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()> {
        self.writeln(s)
    }

    fn write_sep<I, T>(&mut self, items: I, sep: &str) -> std::io::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        self.write_sep(items, sep)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush()
    }
}

/// Batch builder for multiple testcases.
///
/// ```rust
/// use hpdg::io::IOBatchBuilder;
///
/// let batch = IOBatchBuilder::new("case".to_string())
///     .range(1, 3)
///     .data_id_separator("_".to_string())
///     .build();
/// assert_eq!(batch.len(), 3);
/// ```
pub struct IOBatchBuilder {
    prefix: String,
    data_ids: Vec<usize>,
    input_suffix: String,
    output_suffix: String,
    data_id_separator: String,
    data_id_width: Option<usize>,
}

impl IOBatchBuilder {
    /// Create a batch builder rooted at `prefix`.
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            data_ids: Vec::new(),
            input_suffix: "in".to_string(),
            output_suffix: "out".to_string(),
            data_id_separator: "".to_string(),
            data_id_width: None,
        }
    }

    /// Set the explicit testcase ids that should be generated.
    pub fn data_ids<I: IntoIterator<Item = usize>>(mut self, ids: I) -> Self {
        self.data_ids = ids.into_iter().collect();
        self
    }

    /// Use an inclusive numeric range of testcase ids.
    pub fn range(mut self, start: usize, end_inclusive: usize) -> Self {
        self.data_ids = (start..=end_inclusive).collect();
        self
    }

    /// Set the input suffix for every generated [`IO`] value.
    pub fn input_suffix(mut self, input_suffix: String) -> Self {
        self.input_suffix = input_suffix;
        self
    }

    /// Set the output suffix for every generated [`IO`] value.
    pub fn output_suffix(mut self, output_suffix: String) -> Self {
        self.output_suffix = output_suffix;
        self
    }

    /// Set the separator inserted before each testcase id.
    pub fn data_id_separator(mut self, separator: String) -> Self {
        self.data_id_separator = separator;
        self
    }

    /// Set zero-padding width for every testcase id in the batch.
    pub fn data_id_width(mut self, width: Option<usize>) -> Self {
        self.data_id_width = width;
        self
    }

    /// Materialize the configured batch as a list of [`IO`] helpers.
    pub fn build(self) -> Vec<IO> {
        self.data_ids
            .into_iter()
            .map(|id| {
                let mut io = IO::new(self.prefix.clone());
                io.input_suffix(self.input_suffix.clone());
                io.output_suffix(self.output_suffix.clone());
                io.data_id_separator(self.data_id_separator.clone());
                io.data_id_width(self.data_id_width);
                io.data_id(id);
                io
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let testcase_1 = IO::new("moments1".to_string());
        assert_eq!(testcase_1.input_file, "moments1.in".to_string());
        assert_eq!(testcase_1.output_file, "moments1.out".to_string());
        assert_eq!(testcase_1.file_prefix, "moments1".to_string());
        assert_eq!(testcase_1.input_suffix, "in".to_string());
        assert_eq!(testcase_1.output_suffix, "out".to_string());
    }

    #[test]
    fn test_naming_with_data_id() {
        let mut io = IO::new("data".to_string());
        io.data_id_separator("_".to_string())
            .data_id_width(Some(2))
            .data_id(3);
        assert_eq!(io.input_file, "data_03.in".to_string());
        assert_eq!(io.output_file, "data_03.out".to_string());
    }

    #[test]
    fn test_write_and_clear() {
        let mut io = IO::new("buf".to_string());
        io.input_write("1").input_writeln("2");
        assert_eq!(io.input_content, "12\n".to_string());
        io.input_clear();
        assert_eq!(io.input_content, "".to_string());
    }

    #[cfg(feature = "proc")]
    #[test]
    #[ignore]
    fn test_output_gen_basic() {
        use std::fs;
        use std::path::PathBuf;

        let temp_dir = std::env::temp_dir().join("hpdg_io_tests");
        let _ = fs::create_dir_all(&temp_dir);

        let script_path: PathBuf = if cfg!(windows) {
            temp_dir.join("echo_test.bat")
        } else {
            temp_dir.join("echo_test.sh")
        };

        if cfg!(windows) {
            let _ = fs::write(&script_path, "@echo off\r\necho ok\r\n");
        } else {
            let _ = fs::write(&script_path, "#!/bin/sh\necho ok\n");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&script_path).unwrap().permissions();
                perms.set_mode(0o755);
                let _ = fs::set_permissions(&script_path, perms);
            }
        }

        let mut io = IO::new(temp_dir.join("case").to_string_lossy().to_string());
        io.input_write("input");
        let _ = io.output_gen(script_path.to_string_lossy().as_ref());
        assert!(io.output_content.trim().ends_with("ok"));
    }
}
