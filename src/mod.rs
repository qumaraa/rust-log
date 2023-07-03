use colored::*;

#[derive(Copy, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Error,
    Warn,
}

#[derive(Copy, Clone)]
pub struct Logger {
    level: LogLevel,
}


pub impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level as u32 >= self.level as u32 {
            println!("[{}] {}", level_to_string(level.clone()), message);
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
}

fn level_to_string(level: LogLevel) -> String {
    match level {
        LogLevel::Debug => format!("{}", "DEBUG".blue().italic().underline().dimmed()),
        LogLevel::Info => format!("{}", "INFO".green().italic().bold()),
        LogLevel::Error => format!("{}", "ERROR".red().italic().bold()),
        LogLevel::Warn => format!("{}", "WARN".yellow().bold().dimmed()),
    }
}
