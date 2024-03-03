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

pub mod colorize {
    const ANSI_RESET: &str = "\x1b[0m";

    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    pub trait Colorize {
        fn color(&self, clr: Color) -> String;
        fn black(&self) -> String;
        fn red(&self) -> String;
        fn green(&self) -> String;
        fn yellow(&self) -> String;
        fn blue (&self) -> String;
        fn magenta(&self) -> String;
        fn cyan(&self) -> String;
        fn white(&self) -> String;
        fn gray(&self) -> String;
    }

    impl Colorize for &str {
        fn color(&self, clr: Color) -> String {
            let rgb_str = format!("\x1b[{};{};{};0;0m", clr.r.to_string(), clr.g.to_string(), clr.b.to_string());
            format!("{rgb_str}{self}{ANSI_RESET}")
        }

        fn white(&self) -> String {
            self.color(Color {r: 255, g: 255, b: 255})
        }
        
        fn black(&self) -> String {
            self.color(Color {r: 1, g: 1, b: 1})
        }
        
        fn red(&self) -> String {
            self.color(Color { r: 205, g: 49, b: 49,})
        }
        
        fn green(&self) -> String {
            self.color(Color {r: 13, g: 188, b: 121})
        }
        
        fn yellow(&self) -> String {
            self.color(Color {r: 229, g: 229, b: 16})
        }

        fn blue(&self) -> String {
            self.color(Color {r: 36, g: 114, b: 200})
        }
        
        fn magenta(&self) -> String {
            self.color(Color {r: 188, g: 63, b: 188})
        }
        
        fn cyan(&self) -> String {
            self.color(Color {r: 17, g: 168, b: 205})
        }
        
        fn gray(&self) -> String {
            self.color(Color {r: 118,g: 118,b: 118})
        }
    }
}
