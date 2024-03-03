
use crate::colorize::Colorize;
use chrono::prelude::*;

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
    Logger::new(LoggerConfig {
        time: true,

        info_string: "[INFO]  ".to_string(),
        debug_string: "[DEBUG] ".to_string(),
        warn_string: "[WARN!] ".to_string(),
        err_string: "[ERR!]  ".to_string(),

        info_color: LoggerColor { r: 0, g: 0, b: 255 },
        debug_color: LoggerColor {
            r: 148,
            g: 148,
            b: 148,
        },
        warn_color: LoggerColor { r: 0, g: 0, b: 255 },
        err_color: LoggerColor { r: 0, g: 0, b: 255 },
    })
}

pub struct Logger {
    conf: LoggerConfig,
}

impl Logger {
    pub fn new(_conf: LoggerConfig) -> Self {
        Self { conf: _conf }
    }

    pub fn info(&self, msg: String) {
        let mut fmt1 = format!(
            " {} |",
            self.conf.info_string.color(
                self.conf.info_color.r.into(),
                self.conf.info_color.g.into(),
                self.conf.info_color.b.into(),
            )
        );
        if self.conf.time {
            fmt1 = format!(
                "{} | {} |",
                fmt1,
                Utc::now().to_string().color(148, 148, 148)
            );
        }

        println!("{} | {}", fmt1, msg);
    }

    pub fn debug(&self, msg: String) {
        let mut fmt1 = format!(
            " {} |",
            self.conf.debug_string.color(
                self.conf.debug_color.r.into(),
                self.conf.debug_color.g.into(),
                self.conf.debug_color.b.into(),
            )
        );
        if self.conf.time {
            fmt1 = format!(
                "{} | {} |",
                fmt1,
                Utc::now().to_string().color(148, 148, 148)
            );
        }

        println!("{} | {}", fmt1, msg);
    }

    pub fn warn(&self, msg: String) {
        let mut fmt1 = format!(
            " {} |",
            self.conf.warn_string.color(
                self.conf.warn_color.r.into(),
                self.conf.warn_color.g.into(),
                self.conf.warn_color.b.into(),
            )
        );
        if self.conf.time {
            fmt1 = format!(
                "{} | {} |",
                fmt1,
                Utc::now().to_string().color(148, 148, 148)
            );
        }

        println!("{} | {}", fmt1, msg);
    }

    pub fn error(&self, msg: String) {
        let mut fmt1 = format!(
            " {} |",
            self.conf.err_string.color(
                self.conf.err_color.r.into(),
                self.conf.err_color.g.into(),
                self.conf.err_color.b.into(),
            )
        );
        if self.conf.time {
            fmt1 = format!(
                "{} | {} |",
                fmt1,
                Utc::now().to_string().color(148, 148, 148)
            );
        }

        println!("{} | {}", fmt1, msg);
    }
}
