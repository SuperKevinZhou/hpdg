/// IO module for generating testcase input/output buffers and filenames.
#[derive(Debug, Clone)]
pub struct IO {
    input_file: String,
    output_file: String,
    file_prefix: String,
    input_suffix: String,
    output_suffix: String,

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
            input_suffix,
            output_suffix,
            input_content: String::new(),
            output_content: String::new(),
        }
    }

    pub fn input_file(&mut self, input_file: String) -> &mut Self {
        self.input_file = input_file;
        self
    }

    pub fn output_file(&mut self, output_file: String) -> &mut Self {
        self.output_file = output_file;
        self
    }

    pub fn file_prefix(&mut self, file_prefix: String) -> &mut Self {
        self.file_prefix = file_prefix;
        self.rebuild_filenames();
        self
    }

    pub fn input_suffix(&mut self, input_suffix: String) -> &mut Self {
        self.input_suffix = input_suffix.clone();
        self.input_file = format!("{}.{}", self.file_prefix, input_suffix);
        self
    }

    pub fn output_suffix(&mut self, output_suffix: String) -> &mut Self {
        self.output_suffix = output_suffix.clone();
        self.output_file = format!("{}.{}", self.file_prefix, output_suffix);
        self
    }

    fn rebuild_filenames(&mut self) {
        self.input_file = format!("{}.{}", self.file_prefix, self.input_suffix);
        self.output_file = format!("{}.{}", self.file_prefix, self.output_suffix);
    }
}

impl IO {
    pub fn input_write<S: ToString>(&mut self, s: S) -> &mut Self {
        self.input_content.push_str(&s.to_string());
        self
    }

    pub fn output_write<S: ToString>(&mut self, s: S) -> &mut Self {
        self.output_content.push_str(&s.to_string());
        self
    }

    pub fn input_writeln<S: ToString>(&mut self, s: S) -> &mut Self {
        self.input_write(format!("{}\n", s.to_string()));
        self
    }

    pub fn output_writeln<S: ToString>(&mut self, s: S) -> &mut Self {
        self.output_write(format!("{}\n", s.to_string()));
        self
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
