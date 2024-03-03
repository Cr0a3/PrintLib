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

    pub struct Style {
        pub bold: bool,
        pub italic: bool,
        pub underlined: bool,
        pub strike: bool,
    }

    pub trait Colorize {
        fn color(&self, r: i16, g: i16, b: i16) -> String;
        fn black(&self) -> String;
        fn red(&self) -> String;
        fn green(&self) -> String;
        fn yellow(&self) -> String;
        fn blue (&self) -> String;
        fn magenta(&self) -> String;
        fn cyan(&self) -> String;
        fn white(&self) -> String;
        fn gray(&self) -> String;

        fn bold(&self) -> String;
        fn italic(&self) -> String;
        fn underline(&self) -> String;
        fn strike(&self) -> String;
    }

    impl<'a> Colorize for &'a str {

        fn color(&self, r: i16, g: i16, b: i16) -> String {
            ColoredString::new(r, g, b, Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
            }, self).to_string()
        }

        fn white(&self) -> String {
            self.color(255, 255, 255)
        }
        
        fn black(&self) -> String {
            self.color(1, 1, 1)
        }
        
        fn red(&self) -> String {
            self.color(205, 49, 49,)
        }
        
        fn green(&self) -> String {
            self.color(13, 188, 121)
        }
        
        fn yellow(&self) -> String {
            self.color(229, 229, 16)
        }

        fn blue(&self) -> String {
            self.color(36, 114, 200)
        }
        
        fn magenta(&self) -> String {
            self.color(188, 63, 188)
        }
        
        fn cyan(&self) -> String {
            self.color(17, 168, 205)
        }
        
        fn gray(&self) -> String {
            self.color(118, 118, 118)
        }
        
        fn bold(&self) -> String {
            ColoredString::new(-1, -1, -1, Style {
                bold: true,
                italic: false,
                underlined: false,
                strike: false,
            }, self).to_string()
        }
        
        fn italic(&self) -> String {
            ColoredString::new(-1, -1, -1, Style {
                bold: false,
                italic: true,
                underlined: false,
                strike: false,
            }, self).to_string()
        }
        
        fn underline(&self) -> String {
            
            ColoredString::new(-1, -1, -1, Style {
                bold: false,
                italic: false,
                underlined: true,
                strike: false,
            }, self).to_string()
        }
        
        fn strike(&self) -> String {
            
            ColoredString::new(-1, -1, -1, Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: true,
            }, self).to_string()
        }
    }

    impl Colorize for String {

        fn color(&self, r: i16, g: i16, b: i16) -> String {
            ColoredString::new(r, g, b, Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
            }, self).to_string()
        }

        fn white(&self) -> String {
            self.color(255, 255, 255)
        }
        
        fn black(&self) -> String {
            self.color(1, 1, 1)
        }
        
        fn red(&self) -> String {
            self.color(205, 49, 49,)
        }
        
        fn green(&self) -> String {
            self.color(13, 188, 121)
        }
        
        fn yellow(&self) -> String {
            self.color(229, 229, 16)
        }

        fn blue(&self) -> String {
            self.color(36, 114, 200)
        }
        
        fn magenta(&self) -> String {
            self.color(188, 63, 188)
        }
        
        fn cyan(&self) -> String {
            self.color(17, 168, 205)
        }
        
        fn gray(&self) -> String {
            self.color(118, 118, 118)
        }
        
        fn bold(&self) -> String {
            ColoredString::new(-1, -1, -1, Style {
                bold: true,
                italic: false,
                underlined: false,
                strike: false,
            }, self).to_string()
        }
        
        fn italic(&self) -> String {
            ColoredString::new(-1, -1, -1, Style {
                bold: false,
                italic: true,
                underlined: false,
                strike: false,
            }, self).to_string()
        }
        
        fn underline(&self) -> String {
            
            ColoredString::new(-1, -1, -1, Style {
                bold: false,
                italic: false,
                underlined: true,
                strike: false,
            }, self).to_string()
        }
        
        fn strike(&self) -> String {
            
            ColoredString::new(-1, -1, -1, Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: true,
            }, self).to_string()
        }
    }

    pub struct ColoredString {
        r: i16,
        g: i16,
        b: i16,

        attr: Style,
        str: String,
    }

    impl ColoredString {
        pub fn new(_r: i16, _g: i16, _b: i16, _attr: Style, _str: &str) -> Self {
            Self {
                r: _r,
                g: _g,
                b: _b,
                attr: _attr,
                str: _str.into(),
            }
        }
    }

    impl ToString for ColoredString {
        fn to_string(&self) -> String {

            if self.r != -1 {
                let rgb_str = format!("\x1b[38;2;{};{};{}", 
                self.r.to_string(), 
                self.g.to_string(), 
                self.b.to_string());

                return format!("{rgb_str}m{}{ANSI_RESET}", self.str);
            }
            
            let style = &self.attr;

            if style.bold {
                let rgb_str = format!("\x1b[1m");
                return format!("{rgb_str}{}{ANSI_RESET}", self.str);
            }
            if style.italic {
                let rgb_str = format!("\x1b[3m");
                return format!("{rgb_str}{}{ANSI_RESET}", self.str);
            }
            if style.underlined {
                let rgb_str = format!("\x1b[4m");
                return format!("{rgb_str}{}{ANSI_RESET}", self.str);
            }
            if style.strike {
                let rgb_str = format!("\x1b[9m");
                return format!("{rgb_str}{}{ANSI_RESET}", self.str);
            }

            String::new()
        }
    }
}
