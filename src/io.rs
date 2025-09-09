/// IO module for generating testcase input/output buffers and filenames.
pub trait Formatter {
    fn format_item(&self, item: &dyn std::fmt::Display) -> String;
    fn join(&self, items: &[String]) -> String;

    fn format_iter<I, T>(&self, items: I) -> String
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
        Self: Sized,
    {
        let rendered: Vec<String> = items.into_iter().map(|item| self.format_item(&item)).collect();
        self.join(&rendered)
    }
}

#[derive(Debug, Clone)]
pub struct SepFormatter {
    sep: String,
}

impl SepFormatter {
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
        Self { sep: " ".to_string() }
    }
}
#[derive(Debug, Clone)]
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
}

impl IO {
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
        }
    }

    pub fn input_file(&mut self, input_file: String) -> &mut Self {
        self.input_file = Self::normalize_path(&input_file);
        self
    }

    pub fn output_file(&mut self, output_file: String) -> &mut Self {
        self.output_file = Self::normalize_path(&output_file);
        self
    }

    pub fn file_prefix(&mut self, file_prefix: String) -> &mut Self {
        self.file_prefix = file_prefix;
        self.rebuild_filenames();
        self
    }

    pub fn input_prefix(&mut self, input_prefix: String) -> &mut Self {
        self.input_prefix = Some(input_prefix);
        self.rebuild_filenames();
        self
    }

    pub fn output_prefix(&mut self, output_prefix: String) -> &mut Self {
        self.output_prefix = Some(output_prefix);
        self.rebuild_filenames();
        self
    }

    pub fn clear_input_prefix(&mut self) -> &mut Self {
        self.input_prefix = None;
        self.rebuild_filenames();
        self
    }

    pub fn clear_output_prefix(&mut self) -> &mut Self {
        self.output_prefix = None;
        self.rebuild_filenames();
        self
    }

    pub fn data_id(&mut self, data_id: usize) -> &mut Self {
        self.data_id = Some(data_id);
        self.rebuild_filenames();
        self
    }

    pub fn data_id_separator(&mut self, separator: String) -> &mut Self {
        self.data_id_separator = separator;
        self.rebuild_filenames();
        self
    }

    pub fn data_id_width(&mut self, width: Option<usize>) -> &mut Self {
        self.data_id_width = width;
        self.rebuild_filenames();
        self
    }

    pub fn clear_data_id(&mut self) -> &mut Self {
        self.data_id = None;
        self.rebuild_filenames();
        self
    }

    pub fn input_suffix(&mut self, input_suffix: String) -> &mut Self {
        self.input_suffix = input_suffix.clone();
        self.rebuild_filenames();
        self
    }

    pub fn output_suffix(&mut self, output_suffix: String) -> &mut Self {
        self.output_suffix = output_suffix.clone();
        self.rebuild_filenames();
        self
    }

    pub fn input_extension(&mut self, input_extension: String) -> &mut Self {
        self.input_suffix = input_extension;
        self.rebuild_filenames();
        self
    }

    pub fn output_extension(&mut self, output_extension: String) -> &mut Self {
        self.output_suffix = output_extension;
        self.rebuild_filenames();
        self
    }

    pub fn auto_create_dirs(&mut self, enabled: bool) -> &mut Self {
        self.auto_create_dirs = enabled;
        self
    }

    pub fn auto_clean_files(&mut self, enabled: bool) -> &mut Self {
        self.auto_clean_files = enabled;
        self
    }

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
                input_prefix,
                joiner,
                id,
                self.input_suffix
            ));
            self.output_file = Self::normalize_path(&format!(
                "{}{}{}.{}",
                output_prefix,
                joiner,
                id,
                self.output_suffix
            ));
        } else {
            self.input_file = Self::normalize_path(&format!("{}.{}", input_prefix, self.input_suffix));
            self.output_file = Self::normalize_path(&format!("{}.{}", output_prefix, self.output_suffix));
        }
    }

    fn normalize_path(path: &str) -> String {
        let sep = std::path::MAIN_SEPARATOR;
        let mut buf = path.trim().to_string();
        buf = buf.replace(['/', '\\'], sep);
        buf
    }
}

impl IO {
    pub fn input_write<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.input_content, format_args!("{}", s));
        self
    }

    pub fn output_write<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.output_content, format_args!("{}", s));
        self
    }

    pub fn input_writeln<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.input_content, format_args!("{}\n", s));
        self
    }
    
    pub fn output_writeln<S: std::fmt::Display>(&mut self, s: S) -> &mut Self {
        let _ = std::fmt::Write::write_fmt(&mut self.output_content, format_args!("{}\n", s));
        self
    }

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
            let _ = std::fmt::Write::write_fmt(&mut self.input_content, format_args!("{}{}", sep, item));
        }
        self
    }

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
            let _ = std::fmt::Write::write_fmt(&mut self.output_content, format_args!("{}{}", sep, item));
        }
        self
    }

    pub fn input_writeln_sep<I, T>(&mut self, items: I, sep: &str) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        self.input_write_sep(items, sep);
        let _ = std::fmt::Write::write_str(&mut self.input_content, "\n");
        self
    }

    pub fn output_writeln_sep<I, T>(&mut self, items: I, sep: &str) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        self.output_write_sep(items, sep);
        let _ = std::fmt::Write::write_str(&mut self.output_content, "\n");
        self
    }

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

    pub fn input_writeln_slice<T: std::fmt::Display>(&mut self, slice: &[T], sep: &str) -> &mut Self {
        self.input_writeln_sep(slice.iter(), sep)
    }

    pub fn output_writeln_slice<T: std::fmt::Display>(&mut self, slice: &[T], sep: &str) -> &mut Self {
        self.output_writeln_sep(slice.iter(), sep)
    }

    pub fn input_writeln_matrix<T: std::fmt::Display>(&mut self, matrix: &[Vec<T>], sep: &str) -> &mut Self {
        for row in matrix {
            self.input_writeln_sep(row.iter(), sep);
        }
        self
    }

    pub fn output_writeln_matrix<T: std::fmt::Display>(&mut self, matrix: &[Vec<T>], sep: &str) -> &mut Self {
        for row in matrix {
            self.output_writeln_sep(row.iter(), sep);
        }
        self
    }

    pub fn input_clear(&mut self) -> &mut Self {
        self.input_content.clear();
        self.input_bytes.clear();
        self
    }

    pub fn output_clear(&mut self) -> &mut Self {
        self.output_content.clear();
        self.output_bytes.clear();
        self
    }

    pub fn input_write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.input_bytes.extend_from_slice(bytes);
        self
    }

    pub fn output_write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.output_bytes.extend_from_slice(bytes);
        self
    }

    pub fn flush_input_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.input_file)?;
        std::fs::write(&self.input_file, &self.input_content)
    }

    pub fn flush_output_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.output_file)?;
        std::fs::write(&self.output_file, &self.output_content)
    }

    pub fn flush_input_bytes_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.input_file)?;
        std::fs::write(&self.input_file, &self.input_bytes)
    }

    pub fn flush_output_bytes_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.output_file)?;
        std::fs::write(&self.output_file, &self.output_bytes)
    }

    pub fn flush_bytes_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.flush_input_bytes_to_disk()?;
        self.flush_output_bytes_to_disk()?;
        Ok(())
    }

    pub fn flush_to_disk(&self) -> std::io::Result<()> {
        self.ensure_no_conflict()?;
        self.flush_input_to_disk()?;
        self.flush_output_to_disk()?;
        Ok(())
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

    pub fn cleanup_files(&self) -> std::io::Result<()> {
        let _ = std::fs::remove_file(&self.input_file);
        let _ = std::fs::remove_file(&self.output_file);
        Ok(())
    }

    pub fn open_input_stream(&self) -> std::io::Result<IOStream> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.input_file)?;
        let file = std::fs::File::create(&self.input_file)?;
        Ok(IOStream {
            writer: std::io::BufWriter::new(file),
        })
    }

    pub fn open_output_stream(&self) -> std::io::Result<IOStream> {
        self.ensure_no_conflict()?;
        self.prepare_path(&self.output_file)?;
        let file = std::fs::File::create(&self.output_file)?;
        Ok(IOStream {
            writer: std::io::BufWriter::new(file),
        })
    }

    pub fn output_gen(&mut self, program: &str) -> std::io::Result<()> {
        let mut child = std::process::Command::new(program)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(self.input_content.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        self.output_bytes = output.stdout.clone();
        self.output_content = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(())
    }

    pub fn output_gen_with_files(&mut self, program: &str) -> std::io::Result<()> {
        self.flush_input_to_disk()?;
        let input_file = std::fs::File::open(&self.input_file)?;
        let output_file = std::fs::File::create(&self.output_file)?;

        let _ = std::process::Command::new(program)
            .stdin(input_file)
            .stdout(output_file)
            .status()?;

        self.output_bytes = std::fs::read(&self.output_file)?;
        self.output_content = String::from_utf8_lossy(&self.output_bytes).to_string();
        Ok(())
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

pub struct IOStream {
    writer: std::io::BufWriter<std::fs::File>,
}

impl IOStream {
    pub fn write<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()> {
        use std::io::Write;
        write!(self.writer, "{}", s)
    }

    pub fn writeln<S: std::fmt::Display>(&mut self, s: S) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(self.writer, "{}", s)
    }

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

    pub fn flush(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        self.writer.flush()
    }
}

pub struct IOBatchBuilder {
    prefix: String,
    data_ids: Vec<usize>,
    input_suffix: String,
    output_suffix: String,
    data_id_separator: String,
    data_id_width: Option<usize>,
}

impl IOBatchBuilder {
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

    pub fn data_ids<I: IntoIterator<Item = usize>>(mut self, ids: I) -> Self {
        self.data_ids = ids.into_iter().collect();
        self
    }

    pub fn range(mut self, start: usize, end_inclusive: usize) -> Self {
        self.data_ids = (start..=end_inclusive).collect();
        self
    }

    pub fn input_suffix(mut self, input_suffix: String) -> Self {
        self.input_suffix = input_suffix;
        self
    }

    pub fn output_suffix(mut self, output_suffix: String) -> Self {
        self.output_suffix = output_suffix;
        self
    }

    pub fn data_id_separator(mut self, separator: String) -> Self {
        self.data_id_separator = separator;
        self
    }

    pub fn data_id_width(mut self, width: Option<usize>) -> Self {
        self.data_id_width = width;
        self
    }

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
}
