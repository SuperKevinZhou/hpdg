/// IO module for generating testcase input/output buffers and filenames.
#[derive(Debug, Clone)]
pub struct IO {
    input_file: String,
    output_file: String,
    file_prefix: String,
    input_prefix: Option<String>,
    output_prefix: Option<String>,
    data_id: Option<usize>,
    input_suffix: String,
    output_suffix: String,
    auto_create_dirs: bool,
    auto_clean_files: bool,

    input_content: String,
    output_content: String,
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
            input_suffix,
            output_suffix,
            auto_create_dirs: true,
            auto_clean_files: false,
            input_content: String::new(),
            output_content: String::new(),
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

    fn rebuild_filenames(&mut self) {
        let input_prefix = self.input_prefix.as_deref().unwrap_or(&self.file_prefix);
        let output_prefix = self.output_prefix.as_deref().unwrap_or(&self.file_prefix);

        if let Some(data_id) = self.data_id {
            self.input_file = Self::normalize_path(&format!("{}{}.{}", input_prefix, data_id, self.input_suffix));
            self.output_file = Self::normalize_path(&format!("{}{}.{}", output_prefix, data_id, self.output_suffix));
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

    pub fn input_clear(&mut self) -> &mut Self {
        self.input_content.clear();
        self
    }

    pub fn output_clear(&mut self) -> &mut Self {
        self.output_content.clear();
        self
    }

    pub fn flush_input_to_disk(&self) -> std::io::Result<()> {
        if self.auto_create_dirs {
            if let Some(parent) = std::path::Path::new(&self.input_file).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)?;
                }
            }
        }
        if self.auto_clean_files {
            let _ = std::fs::remove_file(&self.input_file);
        }
        std::fs::write(&self.input_file, &self.input_content)
    }

    pub fn flush_output_to_disk(&self) -> std::io::Result<()> {
        if self.auto_create_dirs {
            if let Some(parent) = std::path::Path::new(&self.output_file).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)?;
                }
            }
        }
        if self.auto_clean_files {
            let _ = std::fs::remove_file(&self.output_file);
        }
        std::fs::write(&self.output_file, &self.output_content)
    }

    pub fn flush_to_disk(&self) -> std::io::Result<()> {
        self.flush_input_to_disk()?;
        self.flush_output_to_disk()?;
        Ok(())
    }

    pub fn cleanup_files(&self) -> std::io::Result<()> {
        let _ = std::fs::remove_file(&self.input_file);
        let _ = std::fs::remove_file(&self.output_file);
        Ok(())
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
