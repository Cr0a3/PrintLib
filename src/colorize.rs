
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
    fn blue(&self) -> String;
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
        ColoredString::new(
            r,
            g,
            b,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn white(&self) -> String {
        self.color(255, 255, 255)
    }

    fn black(&self) -> String {
        self.color(1, 1, 1)
    }

    fn red(&self) -> String {
        self.color(205, 49, 49)
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
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: true,
                italic: false,
                underlined: false,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn italic(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: true,
                underlined: false,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn underline(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: true,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn strike(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: true,
            },
            self,
        )
        .to_string()
    }
}

impl Colorize for String {
    fn color(&self, r: i16, g: i16, b: i16) -> String {
        ColoredString::new(
            r,
            g,
            b,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn white(&self) -> String {
        self.color(255, 255, 255)
    }

    fn black(&self) -> String {
        self.color(1, 1, 1)
    }

    fn red(&self) -> String {
        self.color(205, 49, 49)
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
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: true,
                italic: false,
                underlined: false,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn italic(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: true,
                underlined: false,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn underline(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: true,
                strike: false,
            },
            self,
        )
        .to_string()
    }

    fn strike(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: true,
            },
            self,
        )
        .to_string()
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
            let rgb_str = format!(
                "\x1b[38;5;{};{};{}",
                self.r.to_string(),
                self.g.to_string(),
                self.b.to_string()
            );

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
