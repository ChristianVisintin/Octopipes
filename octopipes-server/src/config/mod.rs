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

//Types
pub struct Config {
    pub log_config: LogConfig,
    pub pipes_config: PipesConfig,
    pub protocol_config: ProtocolConfig,
}

pub struct LogConfig {
    pub log_level: isize,
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

//Implementation
impl Config {
    //pub fn parse_config(config_file: String) -> Config {
    //TODO: implement
    //}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config() {}
}
