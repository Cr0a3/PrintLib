use colored::Colorize;

pub mod error {
    use colored::Colorize;


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
    
        pub fn add_code_line(&mut self, line: String, display_line_no: bool, line_no: usize, display_add: bool) {
            let mut code_line = String::new();
    
            if display_line_no {
                code_line += line_no.to_string().as_str();
            }
    
            else if display_add {
                code_line += "+++";
            }
    
            code_line += " | ";
    
            self.before_len = code_line.clone().len() -1;
    
            code_line += line.as_str();
    
            self.fmt_lines.push(code_line);
        }
    
        pub fn add_where(&mut self, where_start: usize, where_length: usize, where_msg_b: bool, where_msg: String) {
            let mut where_str = String::new();
    
            where_str += " ".repeat(where_start + self.before_len).as_str();
            
            if where_msg_b {
                where_str += format!("^{}", where_msg).as_str();
            }
    
            else {
                where_str += "^".repeat(where_length).as_str();
            }
    
            self.fmt_lines.push(where_str);
        }
    
        pub fn add_arrow(&mut self, file: String, line: usize, where_start: usize) {
            let arrow = 
                                format!("  -->{}:{}:{}", file, line, where_start);
    
            self.fmt_lines.push(arrow);
        }
    
        pub fn add_arrowW(&mut self, file: String, line: usize) {
            let arrow = 
                                format!("  -->{}:{}", file, line);
    
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
}

pub mod logr {
    use chrono::prelude::*;
    use colored::Colorize;

    pub struct LoggerColor {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    pub struct LoggerConfig {
        pub time: bool,
        
        pub info_string: String,
        pub info_color: LoggerColor,

        pub debug_string: String,
        pub debug_color: LoggerColor,

        pub warn_string: String,
        pub warn_color: LoggerColor,

        pub err_string: String,
        pub err_color: LoggerColor,
    }

    pub fn def() -> Logger {
        Logger::new( 
            LoggerConfig {
                time: true,

                info_string:    "[INFO]  ".to_string(),
                debug_string:   "[DEBUG] ".to_string(),
                warn_string:    "[WARN!] ".to_string(),
                err_string:     "[ERR!]  ".to_string(),

                info_color: LoggerColor { r: 0,  g: 0,  b: 255 },
                debug_color: LoggerColor { r: 148, g: 148, b: 148},
                warn_color: LoggerColor { r: 0,  g: 0,  b: 255 },
                err_color: LoggerColor { r: 0,  g: 0,  b: 255 },
            }
        )
    }

    pub struct Logger {
        conf: LoggerConfig,
    }

    impl Logger {
        pub fn new(_conf: LoggerConfig) -> Self {
            Self {
                conf: _conf
            }
        }

        pub fn info(&self, msg: String) {
            let mut fmt1 = format!(" {} |", self.conf.info_string.truecolor(
                self.conf.info_color.r, 
                self.conf.info_color.g,
                self.conf.info_color.b,
            ));
            if self.conf.time {
                fmt1 = format!(
                    "{} | {} |", fmt1, Utc::now().to_string().truecolor(148, 148, 148)
                );
            }
            
            println!("{} | {}", fmt1, msg);
        }

        pub fn debug(&self, msg: String) {
            let mut fmt1 = format!(" {} |", self.conf.debug_string.truecolor(
                self.conf.debug_color.r, 
                self.conf.debug_color.g,
                self.conf.debug_color.b,
            ));
            if self.conf.time {
                fmt1 = format!(
                    "{} | {} |", fmt1, Utc::now().to_string().truecolor(148, 148, 148)
                );
            }
            
            println!("{} | {}", fmt1, msg);
        }

        pub fn warn(&self, msg: String) {
            let mut fmt1 = format!(" {} |", self.conf.warn_string.truecolor(
                self.conf.warn_color.r, 
                self.conf.warn_color.g,
                self.conf.warn_color.b,
            ));
            if self.conf.time {
                fmt1 = format!(
                    "{} | {} |", fmt1, Utc::now().to_string().truecolor(148, 148, 148)
                );
            }
            
            println!("{} | {}", fmt1, msg);
        }

        pub fn error(&self, msg: String) {
            let mut fmt1 = format!(" {} |", self.conf.err_string.truecolor(
                self.conf.err_color.r, 
                self.conf.err_color.g,
                self.conf.err_color.b,
            ));
            if self.conf.time {
                fmt1 = format!(
                    "{} | {} |", fmt1, Utc::now().to_string().truecolor(148, 148, 148)
                );
            }
            
            println!("{} | {}", fmt1, msg);
        }
    }
}
