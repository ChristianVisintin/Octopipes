//! ### logger
//!
//! `logger` is the module which implements Log and is used to handle log for Octopipes

//
//   Octopipes-Server
//   Developed by Christian Visintin
//
// MIT License
// Copyright (c) 2020 Christian Visintin
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

extern crate chrono;
extern crate log;

use chrono::{Datelike, Local, Timelike};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum OctoLogLevel {
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

pub struct OctoLogger {
    enabled: bool,
    level: log::Level,
    file: String,
    stdout: bool,
}

impl OctoLogLevel {
    /// ### OctoLogLevel::from_int
    ///
    /// Convert an integer to OctoLogLevel
    pub fn from_int(level: usize) -> OctoLogLevel {
        match level {
            1 => OctoLogLevel::Debug,
            2 => OctoLogLevel::Info,
            3 => OctoLogLevel::Warn,
            4 => OctoLogLevel::Error,
            _ => OctoLogLevel::Error,
        }
    }
}

impl OctoLogger {
    /// ### OctoLogger::new
    ///
    /// Instantiates a new OctoLogger struct with the provided parameters
    pub fn new(enabled: bool, level: OctoLogLevel, file: String, stdout: bool) -> OctoLogger {
        let log_level: log::Level = OctoLogger::level_from_int(level);
        OctoLogger {
            enabled: enabled,
            level: log_level,
            file: file,
            stdout: stdout,
        }
    }

    /// ### debug
    ///
    /// Log a debug message
    pub fn debug(&self, args: std::fmt::Arguments) {
        let mut metadata_builder = log::MetadataBuilder::new();
        metadata_builder.level(log::Level::Debug);
        let mut record_builder = log::RecordBuilder::new();
        record_builder.args(args);
        record_builder.metadata(metadata_builder.build());
        let log_record = record_builder.build();
        log::Log::log(self, &log_record);
    }

    /// ### info
    ///
    /// Log a info message
    pub fn info(&self, args: std::fmt::Arguments) {
        let mut metadata_builder = log::MetadataBuilder::new();
        metadata_builder.level(log::Level::Info);
        let mut record_builder = log::RecordBuilder::new();
        record_builder.args(args);
        record_builder.metadata(metadata_builder.build());
        let log_record = record_builder.build();
        log::Log::log(self, &log_record);
    }

    /// ### wann
    ///
    /// Log a warn message
    pub fn warn(&self, args: std::fmt::Arguments) {
        let mut metadata_builder = log::MetadataBuilder::new();
        metadata_builder.level(log::Level::Warn);
        let mut record_builder = log::RecordBuilder::new();
        record_builder.args(args);
        record_builder.metadata(metadata_builder.build());
        let log_record = record_builder.build();
        log::Log::log(self, &log_record);
    }

    /// ### error
    ///
    /// Log an error message
    pub fn error(&self, args: std::fmt::Arguments) {
        let mut metadata_builder = log::MetadataBuilder::new();
        metadata_builder.level(log::Level::Error);
        let mut record_builder = log::RecordBuilder::new();
        record_builder.args(args);
        record_builder.metadata(metadata_builder.build());
        let log_record = record_builder.build();
        log::Log::log(self, &log_record);
    }

    /// ### level_from_int
    ///
    /// Converts a OctoLogLevel to LogLevel
    fn level_from_int(log_level_int: OctoLogLevel) -> log::Level {
        match log_level_int {
            OctoLogLevel::Debug => log::Level::Debug,
            OctoLogLevel::Info => log::Level::Info,
            OctoLogLevel::Warn => log::Level::Warn,
            OctoLogLevel::Error => log::Level::Error,
        }
    }

    fn format_time() -> String {
        let t_now = Local::now();
        String::from(
            [
                t_now.year().to_string(),
                String::from("/"),
                t_now.month().to_string(),
                String::from("/"),
                t_now.day().to_string(),
                String::from("-"),
                t_now.hour().to_string(),
                String::from(":"),
                t_now.minute().to_string(),
                String::from(":"),
                t_now.second().to_string(),
            ]
            .join(""),
        )
    }
}

impl log::Log for OctoLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if !self.enabled {
            return false;
        }
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            if self.stdout {
                //IF stdout is enabled, write to stdout
                println!(
                    "{} [{}]: {}",
                    OctoLogger::format_time(),
                    record.level(),
                    record.args()
                );
            }
            //Write to file
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.file.as_str())
            {
                if let Err(_) = writeln!(
                    file,
                    "{} [{}]: {}",
                    OctoLogger::format_time(),
                    record.level(),
                    record.args()
                ) {
                    if self.stdout {
                        println!(
                            "{} [{}]: Could not open file {}",
                            OctoLogger::format_time(),
                            record.level(),
                            self.file.as_str()
                        );
                    }
                }
            }
        } else {
            if self.stdout {
                println!(
                    "{} [{}]: Could not open file {}",
                    OctoLogger::format_time(),
                    record.level(),
                    self.file.as_str()
                );
            }
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Read;

    #[test]
    fn test_logger() {
        //Instance logger
        let mut tmp_log_file: tempfile::NamedTempFile = generate_log_file();
        let tmp_log_file_path: String = String::from(tmp_log_file.path().to_str().unwrap());
        let log_level: OctoLogLevel = OctoLogLevel::from_int(1);
        assert_eq!(log_level, OctoLogLevel::Debug);
        let logger: OctoLogger = OctoLogger::new(true, log_level, tmp_log_file_path, true);
        //Test to log a DEBUG, INFO, WARN and ERROR message
        logger.debug(format_args!("Testing a {} message", "DEBUG"));
        logger.info(format_args!("Testing a {} message", "INFO"));
        logger.warn(format_args!("Testing a {} message", "WARN"));
        logger.error(format_args!("Testing a {} message", "ERROR"));
        //Read log file and verify its content
        let mut log_content: String = String::with_capacity(2048);
        tmp_log_file.read_to_string(&mut log_content).unwrap();
        println!("Read log file content: '{}'", log_content);
        assert_eq!(log_content.len(), String::from("2020/2/8-17:45:35 [DEBUG]: Testing a DEBUG message\n2020/2/8-17:45:35 [INFO]: Testing a INFO message\n2020/2/8-17:45:35 [WARN]: Testing a WARN message\n2020/2/8-17:45:35 [ERROR]: Testing a ERROR message\n").len());
    }

    #[test]
    fn test_logger_disabled() {
        //Instance logger
        let mut tmp_log_file: tempfile::NamedTempFile = generate_log_file();
        let tmp_log_file_path: String = String::from(tmp_log_file.path().to_str().unwrap());
        let log_level: OctoLogLevel = OctoLogLevel::from_int(3);
        assert_eq!(log_level, OctoLogLevel::Warn);
        //Only WARN level is used
        let logger: OctoLogger = OctoLogger::new(true, log_level, tmp_log_file_path, true);
        //Test to log a DEBUG, INFO, WARN and ERROR message
        logger.debug(format_args!("Testing a {} message", "DEBUG"));
        logger.info(format_args!("Testing a {} message", "INFO"));
        logger.warn(format_args!("Testing a {} message", "WARN"));
        logger.error(format_args!("Testing a {} message", "ERROR"));
        //Read log file and verify its content
        let mut log_content: String = String::with_capacity(2048);
        tmp_log_file.read_to_string(&mut log_content).unwrap();
        println!("Read log file content: '{}'", log_content);
        assert_eq!(log_content.len(), String::from("2020/2/8-17:45:35 [WARN]: Testing a WARN message\n2020/2/8-17:45:35 [ERROR]: Testing a ERROR message\n").len());
    }

    /// ### write_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn generate_log_file() -> tempfile::NamedTempFile {
        // Create file
        let tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        tmpfile
    }
}
