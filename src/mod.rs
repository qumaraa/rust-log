//!
//! # Rust-Log: A simple and flexible Logging System for Rust.
//! We present you a simple but flexible logging system with many different levels of logging.
//!
//! [LogLevel] - structure containing logging levels
//!
//! [Logger] - a structure containing functions for logging.
//!
//! # Example
//! ```
//!fn main() {
//!   use rust_log::{Logger, LogLevel};
//!
//!   let logger = Logger::new(LogLevel::Debug, "c:/users/user/desktop/my.txt", true);
//!
//!   // use macros (more comfortable)
//!
//!   info!(logger, "This is an info message!");
//!   /*...*/
//!}
//!
//! ```
//!
//!
#[macro_use]
#[macro_export]
extern crate colored;

use std::fs;
use std::io::Write;
use chrono::Utc;
use colored::*;

#[derive(Copy, Clone,PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Error,
    Warn,
    Fatal,
    Critical,
}

#[derive(Copy, Clone,PartialEq)]
pub struct Logger {
    level: LogLevel,
    path: &'static str,
    save: bool,
}

impl Logger {
    pub fn new(level: LogLevel, path: &'static str, save: bool) -> Self {
        Logger { level, path, save }
    }

    fn log(&mut self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 && level != LogLevel::Error {
            let now = Utc::now();
            let msg = format!("[{}] {} - {}\n", level_to_string(level.clone()), now, message);
            println!("{msg}");
            if self.save {
                self.save_log(msg.as_str());
            }
        }else {
            let now = Utc::now();
            let msg = format!("[{}] {} - {}\n", level_to_string(level.clone()), now, message);
            println!("{msg}");
            if self.save {
                self.save_log(msg.as_str());
            }
        }
    }
    fn save_log(&mut self, data: &str) {
        let mut op = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.path).unwrap();
        op.write_all(data.as_bytes()).unwrap();
    }
    pub fn debug(&mut self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn warn(&mut self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn fatal(&mut self, message: &str) {
        self.log(LogLevel::Fatal, message);
    }

    pub fn critical(&mut self, message: &str) {
        self.log(LogLevel::Critical, message);
    }
}

macro_rules! debug {
    ($logger:expr, $message:expr) => {
        $logger.debug($message);
    };
}

macro_rules! warn {
    ($logger:expr, $message:expr) => {
        $logger.warn($message);
    };
}

macro_rules! error {
    ($logger:expr, $message:expr) => {
        $logger.error($message);
    };
}

macro_rules! info {
    ($logger:expr, $message:expr) => {
        $logger.info($message);
    };
}

macro_rules! fatal {
    ($logger:expr, $message:expr) => {
        $logger.fatal($message);
    };
}

macro_rules! critical {
    ($logger:expr, $message:expr) => {
        $logger.critical($message);
    };
}

fn level_to_string(level: LogLevel) -> String {
    match level {
        LogLevel::Debug => format!("{}", "DEBUG".cyan().italic().underline()),
        LogLevel::Info => format!("{}", "INFO".green().bold()),
        LogLevel::Error => format!("{}", "ERROR".red().italic().bold()),
        LogLevel::Warn => format!("{}", "WARN".yellow().on_yellow().bold().dimmed()),
        LogLevel::Fatal => format!("{}", "FATAL".on_bright_red().white().bold().underline()),
        LogLevel::Critical => format!("{}", "CRITICAL".bright_red().bold().italic().underline()),
    }
}


