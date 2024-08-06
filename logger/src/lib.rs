/*
logger/src/lib.rs
6/29/24

This file defines a simple logger.
The output is written to log.txt
*/
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use chrono::Local;

// Supported log levels
#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

// Logger structure
pub struct Logger {
    level: LogLevel,
    file: Arc<Mutex<std::fs::File>>,
}

impl Logger {
    // Creates a new Logger with a specified log level and output file
    pub fn new(level: LogLevel, file_path: &str) -> Logger {
        let initial_message = r"
          ____ _____      _______
         / __//_  _/ ,/| /____   )
        ( (    / / ,/  |   __ ) /
         \ \  / /,/ _  |  / // /
      ____) )/ //,-' `.| / / \ \
     /_____//_///     ||/_/   \ \_ ______ ______     ______  __   __
                               \_//_  __//___   )   / ____/ / /  / /
                                   / /    __ ) /   / /_    / / /'/'
                                  / /    / // /   / ___|  / //'/'
                                 / /    / / \ \  / /____ / / \ \
                                /_/    /_/   \ \/______//_/   \_\
                                              \/
        ";
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();

        {
            // Write the initial message to the file
            let mut file = file.try_clone().unwrap();
            writeln!(file, "{}", initial_message).unwrap();
        }

        Logger {
            level,
            file: Arc::new(Mutex::new(file)),
        }
    }

    // Writes a log to the file
    fn log(&self, level: &str, message: &str) {
        let mut file = self.file.lock().unwrap();
        let now = Local::now();
        writeln!(file, "[{}] [{}]: {}", now.format("%Y-%m-%d %H:%M:%S"), level, message).unwrap();
    }

    // Logs an info message
    pub fn info(&self, message: &str) {
        if matches!(self.level, LogLevel::Info) {
            self.log("INFO", message);
        }
    }

    // Logs a warning message
    pub fn warning(&self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Warning) {
            self.log("WARNING", message);
        }
    }

    // Logs an error message
    pub fn error(&self, message: &str) {
        self.log("ERROR", message);
    }
}

// A macro to simplify logging at different levels
#[macro_export]
macro_rules! log {
    ($logger:expr, $level:ident, $($arg:tt)*) => {
        match stringify!($level) {
            "info" => $logger.info(&format!($($arg)*)),
            "warning" => $logger.warning(&format!($($arg)*)),
            "error" => $logger.error(&format!($($arg)*)),
            _ => panic!("Unknown log level"),
        }
    };
}
