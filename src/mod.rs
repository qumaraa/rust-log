extern crate colored;
use colored::*;
use chrono::Utc;

#[derive(Copy, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Error,
    Warn,
    Fatal,
    Critical,
}


#[derive(Copy, Clone)]
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 {
            let now = Utc::now();
            println!("[{}] {} - {}", level_to_string(level.clone()),now,message);
        }
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn fatal(&self, message: &str) {
        self.log(LogLevel::Fatal, message);
    }
    pub fn critical(&self, message: &str) {
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
