//! ### config
//!
//! `config` is the module which handles the octopipes server configuration

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

extern crate yaml_rust;

use std::fmt;
use yaml_rust::{Yaml, YamlLoader};

//Types
pub struct Config {
    pub log_config: LogConfig,
    pub pipes_config: PipesConfig,
    pub protocol_config: ProtocolConfig,
}

pub struct LogConfig {
    pub log_level: usize,
    pub log_file: String,
    pub stdout: bool,
}

pub struct PipesConfig {
    pub cap_path: String,
    pub client_dir: String,
}

pub struct ProtocolConfig {
    pub version: u8,
}

#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum ConfigErrorCode {
    NoSuchFileOrDirectory,
    CouldNotReadFile,
    YamlSyntaxError,
}

pub struct ConfigError {
    pub code: ConfigErrorCode,
    pub message: String,
}

//Implementation
impl Config {
    /// ### parse_config
    ///
    /// `parse_config` parse a YAML configuration file and return a Config struct
    pub fn parse_config(config_file: String) -> Result<Config, ConfigError> {
        let config_str: String;
        //Read configuration file
        match std::fs::read_to_string(config_file.clone()) {
            Ok(config) => config_str = config,
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::NotFound => {
                        return Err(ConfigError {
                            code: ConfigErrorCode::NoSuchFileOrDirectory,
                            message: String::from(["No such file or directory: ", config_file.as_str()].join(" ")),
                        })
                    },
                    _ => {
                        return Err(ConfigError {
                            code: ConfigErrorCode::CouldNotReadFile,
                            message: String::from(["Could not read file ", config_file.as_str()].join(" ")),
                        })
                    }
                }
                
            }
        };
        //Parse YAML file
        let yaml_docs: Vec<Yaml>;
        match YamlLoader::load_from_str(config_str.as_str()) {
            Ok(doc) => yaml_docs = doc,
            Err(_) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from(["Could not parse file", config_file.as_str()].join(" ")),
                })
            }
        };
        //Check there is at least one document
        if yaml_docs.len() == 0 {
            return Err(ConfigError {
                code: ConfigErrorCode::YamlSyntaxError,
                message: String::from("File does not contain any YAML document"),
            });
        };
        let yaml_doc: &Yaml = &yaml_docs[0];
        //Look for keys and get configuration parts
        let logging_config_yaml = &yaml_doc["logging"];
        let pipes_config_yaml = &yaml_doc["pipes"];
        let protocol_config_yaml = &yaml_doc["protocol"];
        let logging_config: LogConfig = match LogConfig::parse_log_config(logging_config_yaml) {
            Ok(config) => config,
            Err(err) => return Err(err),
        };
        let pipes_config: PipesConfig = match PipesConfig::parse_pipes_config(pipes_config_yaml) {
            Ok(config) => config,
            Err(err) => return Err(err),
        };
        let protocol_config: ProtocolConfig =
            match ProtocolConfig::parse_protocol_config(protocol_config_yaml) {
                Ok(config) => config,
                Err(err) => return Err(err),
            };
        Ok(Config {
            log_config: logging_config,
            pipes_config: pipes_config,
            protocol_config: protocol_config,
        })
    }
}

impl LogConfig {
    /// ### parse_log_config
    ///
    /// `parse_log_config` parse a YAML document and get LogConfig
    fn parse_log_config(config_doc: &Yaml) -> Result<LogConfig, ConfigError> {
        let log_level: usize = match config_doc["log_level"].as_i64() {
            Some(value) => value as usize,
            None => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from("Could not find 'log_level' in 'logging'"),
                })
            }
        };
        let log_file: String = match config_doc["log_file"].as_str() {
            Some(value) => String::from(value),
            None => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from("Could not find 'log_file' in 'logging'"),
                })
            }
        };
        let stdout: bool = match config_doc["stdout"].as_bool() {
            Some(value) => value,
            None => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from("Could not find 'stdout' in 'logging'"),
                })
            }
        };
        Ok(LogConfig {
            log_level: log_level,
            log_file: log_file,
            stdout: stdout,
        })
    }
}

impl PipesConfig {
    /// ### parse_pipes_config
    ///
    /// `parse_pipes_config` parse a YAML document and get PipesConfig
    fn parse_pipes_config(config_doc: &Yaml) -> Result<PipesConfig, ConfigError> {
        let cap_path: String = match config_doc["cap_path"].as_str() {
            Some(value) => String::from(value),
            None => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from("Could not find 'cap_path' in 'pipes'"),
                })
            }
        };
        let client_dir: String = match config_doc["client_dir"].as_str() {
            Some(value) => String::from(value),
            None => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from("Could not find 'client_dir' in 'pipes'"),
                })
            }
        };
        Ok(PipesConfig {
            cap_path: cap_path,
            client_dir: client_dir,
        })
    }
}

impl ProtocolConfig {
    /// ### parse_protocol_config
    ///
    /// `parse_protocol_config` parse a YAML document and get ProtocolConfig
    fn parse_protocol_config(config_doc: &Yaml) -> Result<ProtocolConfig, ConfigError> {
        let protocol_version: u8 = match config_doc["version"].as_i64() {
            Some(value) => value as u8,
            None => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from("Could not find 'version' in 'protocol'"),
                })
            }
        };
        Ok(ProtocolConfig {
            version: protocol_version,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Write;

    #[test]
    fn test_config() {
        //Try to parse a configuration file
        let config_file: tempfile::NamedTempFile = write_config_file();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        match Config::parse_config(config_file_path) {
            Ok(config) => {
                //Verify config parameters
                //Log config
                assert_eq!(config.log_config.log_file, String::from("/var/log/octopipes/octopipes.log"));
                assert_eq!(config.log_config.log_level, 1);
                assert_eq!(config.log_config.stdout, true);
                //Pipes config
                assert_eq!(config.pipes_config.cap_path, String::from("/tmp/octopipes/cap.pipe"));
                assert_eq!(config.pipes_config.client_dir, String::from("/tmp/octopipes/clients/"));
                //Protocol config
                assert_eq!(config.protocol_config.version, 1);
            },
            Err(error) => {
                panic!("Parse_config should have returned OK, but returned {} ({:?})", error.message, error.code)
            }
        };
    }

    #[test]
    fn test_config_no_such_file() {
        if let Err(error) = Config::parse_config(String::from("unexisting_config.yml")) {
            assert_eq!(error.code, ConfigErrorCode::NoSuchFileOrDirectory)
        } else {
            panic!("parse_config of a not existing file returned Ok");
        }
    }

    #[test]
    fn test_config_bad_syntax() {
        //No protocol
        let config_file: tempfile::NamedTempFile = write_config_file_bad1();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        if let Err(error) = Config::parse_config(String::from(config_file_path)) {
            assert_eq!(error.code, ConfigErrorCode::YamlSyntaxError);
        } else {
            panic!("parse_config of a config file with bad syntax returned Ok");
        }
        //No Pipes
        let config_file: tempfile::NamedTempFile = write_config_file_bad2();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        if let Err(error) = Config::parse_config(String::from(config_file_path)) {
            assert_eq!(error.code, ConfigErrorCode::YamlSyntaxError);
        } else {
            panic!("parse_config of a config file with bad syntax returned Ok");
        }
        //No logging
        let config_file: tempfile::NamedTempFile = write_config_file_bad3();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        if let Err(error) = Config::parse_config(String::from(config_file_path)) {
            assert_eq!(error.code, ConfigErrorCode::YamlSyntaxError);
        } else {
            panic!("parse_config of a config file with bad syntax returned Ok");
        }
    }

    /// ### write_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_file() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "logging:\n    enabled: true\n    log_level: 1\n    log_file: \"/var/log/octopipes/octopipes.log\"\n    stdout: true\npipes:\n    cap_path: \"/tmp/octopipes/cap.pipe\"\n    client_dir: \"/tmp/octopipes/clients/\"\nprotocol:\n    version: 1\n").unwrap();
        tmpfile
    }

    /// ### write_bad_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_file_bad1() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "logging:\n    enabled: true\n    log_level: 1\n    log_file: \"/var/log/octopipes/octopipes.log\"\n    stdout: true\npipes:\n    cap_path: \"/tmp/octopipes/cap.pipe\"\n    client_dir: \"/tmp/octopipes/clients/\"\n").unwrap();
        tmpfile
    }

    /// ### write_bad_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_file_bad2() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "logging:\n    enabled: true\n    log_level: 1\n    log_file: \"/var/log/octopipes/octopipes.log\"\n    stdout: true\nprotocol:\n    version: 1\n").unwrap();
        tmpfile
    }

    /// ### write_bad_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_file_bad3() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "pipes:\n    cap_path: \"/tmp/octopipes/cap.pipe\"\n    client_dir: \"/tmp/octopipes/clients/\"\nprotocol:\n    version: 1\n").unwrap();
        tmpfile
    }
}
