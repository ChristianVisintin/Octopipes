//! # Octopipes-Server
//!
//! `octopipes-server` is the official server which implements the Octopipes Protocol.

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

extern crate ctrlc;
extern crate getopts;
extern crate rustypipes;

use getopts::Options;
use rustypipes::{OctopipesCapError, OctopipesCapMessage, OctopipesProtocolVersion, OctopipesServer, OctopipesServerError};
use std::env;
use std::process::exit;
use std::sync::mpsc;

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();
    //Program options
    //Get opts
    let mut opts = Options::new();
    opts.optopt(
        "C",
        "configuration file",
        "Specify the configuration YAML file",
        "<config_yaml>",
    );
    opts.optopt("c", "cap-path", "Specify CAP path", "<CAP_PATH>");
    opts.optopt(
        "d",
        "client-dir",
        "Specify the client's pipes directory",
        "<client_dir>",
    );
    opts.optopt(
        "l",
        "log-level",
        "Specify the server's log level (0: NONE, 1: DEBUG, 2: INFO, 3: WARN, 4: ERROR)",
        "<log_level>",
    );
    opts.optopt(
        "L",
        "log-file",
        "Specify the server's log file",
        "<log_file>",
    );
    opts.optopt(
        "P",
        "pidfile",
        "Specify the file where the pidfile will be written",
        "<pidfile>"
    );
    opts.optflag("h", "help", "print this help menu");
}
