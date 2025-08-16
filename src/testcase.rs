pub struct Testcase {
    input_file: String,
    output_file: String,
    io_prefix: String,
    input_subfix: String,
    output_subfix: String,
    
    input_content: String,
    output_content: String,
}

impl Testcase {
    pub fn new(io_prefix: String) -> Testcase {
        Testcase {
            input_file: format!("{}.in", io_prefix),
            output_file: format!("{}.out", io_prefix),
            io_prefix,
            input_subfix: "in".to_string(),
            output_subfix: "out".to_string(),

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

    pub fn io_prefix(&mut self, io_prefix: String) -> &mut Self {
        self.io_prefix = io_prefix;
        self
    }

    pub fn input_subfix(&mut self, input_subfix: String) -> &mut Self {
        self.input_subfix = input_subfix.clone();
        self.input_file = format!("{}.{}", self.io_prefix, input_subfix);
        self
    }

    pub fn output_subfix(&mut self, output_subfix: String) -> &mut Self {
        self.output_subfix = output_subfix.clone();
        self.output_file = format!("{}.{}", self.io_prefix, output_subfix);
        self
    }
}

impl Testcase {
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
        let testcase_1 = Testcase::new("moments1".to_string());
        assert_eq!(testcase_1.input_file, "moments1.in".to_string());
        assert_eq!(testcase_1.output_file, "moments1.out".to_string());
        assert_eq!(testcase_1.io_prefix, "moments1".to_string());
        assert_eq!(testcase_1.input_subfix, "in".to_string());
        assert_eq!(testcase_1.output_subfix, "out".to_string());
    }
}
