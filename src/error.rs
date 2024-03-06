use crate::colorize::Colorize;

pub struct ErrorFactory {
    ecode: String,
    msg: String,
    fmt_lines: Vec<String>,
    before_len: usize,
}

impl ErrorFactory {
    pub fn new(_ecode: String, _msg: String) -> Self {
        ErrorFactory {
            ecode: _ecode,
            msg: _msg,
            fmt_lines: Vec::new(),
            before_len: 3,
        }
    }

    pub fn add_code_line(
        &mut self,
        line: String,
        display_line_no: bool,
        line_no: usize,
        display_add: bool,
    ) {
        let mut code_line = String::new();

        if display_line_no {
            code_line += line_no.to_string().as_str();
        } else if display_add {
            code_line += "+++";
        }

        code_line += " | ";

        self.before_len = code_line.clone().len() - 1;

        code_line += line.as_str();

        self.fmt_lines.push(code_line);
    }

    pub fn add_where(
        &mut self,
        where_start: usize,
        where_length: usize,
        where_msg_b: bool,
        where_msg: String,
    ) {
        let mut where_str = String::new();

        where_str += " ".repeat(where_start + self.before_len).as_str();

        if where_msg_b {
            where_str += format!("^{}", where_msg).as_str();
        } else {
            where_str += "^".repeat(where_length).as_str();
        }

        self.fmt_lines.push(where_str);
    }

    pub fn add_arrow(&mut self, file: String, line: usize, where_start: usize) {
        let arrow = format!("  -->{}:{}:{}", file, line, where_start);

        self.fmt_lines.push(arrow);
    }

    pub fn add_arrow_w(&mut self, file: String, line: usize) {
        let arrow = format!("  -->{}:{}", file, line);

        self.fmt_lines.push(arrow);
    }

    pub fn print(&self) {
        let fmt_error = format!("error[{}]", self.ecode).red();
        println!("{}: {}", fmt_error, self.msg.bold());

        //print out all elements of self.fmt_lines
        for line in self.fmt_lines.iter() {
            println!("{}", line);
        }
    }
}
