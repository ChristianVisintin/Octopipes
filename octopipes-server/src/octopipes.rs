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

const OCTOPIPES_SERVER_VERSION: &str = "0.1.0";

use getopts::Options;
use rustypipes::{OctopipesProtocolVersion, OctopipesServer};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;

mod config;
mod logger;

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn write_pid(pid_file: String, pid: u32) -> Result<u32, std::io::Error> {
    match OpenOptions::new().create(true).write(true).open(pid_file) {
        Ok(mut file_hnd) => {
            //Write pid to file
            if let Err(ioerr) = write!(file_hnd, "{}", pid) {
                return Err(ioerr);
            }
            Ok(pid)
        }
        Err(io_err) => return Err(io_err),
    }
}

fn int_to_protocol_version(version: u8) -> OctopipesProtocolVersion {
    match version {
        1 => OctopipesProtocolVersion::Version1,
        _ => OctopipesProtocolVersion::Version1,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();
    //Program CLI options
    //Pidfile
    let mut pid_file: Option<String> = None;
    //Config
    let configuration_file: String;
    //Pipes
    let mut cap_path: Option<String> = None; //Overridable
    let mut client_dir: Option<String> = None; //Overridable
                                               //Logging
    let mut log_level: Option<usize> = None; //Overridable
    let mut log_file: Option<String> = None; //Overridable
                                             //@! Get opts
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
        "<pidfile>",
    );
    opts.optflag("h", "help", "print this help menu");
    //Get options
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(255);
    };
    //Let's start with the configuration
    match matches.opt_str("C") {
        Some(cfg_file) => {
            configuration_file = cfg_file;
        }
        None => {
            println!("Missing Configuration file!");
            print_usage(&program, opts);
            std::process::exit(255);
        }
    };
    if let Some(cap) = matches.opt_str("c") {
        cap_path = Some(cap);
    };
    if let Some(cli_dir) = matches.opt_str("d") {
        client_dir = Some(cli_dir);
    };
    if let Some(log_level_str) = matches.opt_str("l") {
        log_level = Some(log_level_str.parse().unwrap());
    };
    if let Some(lfile) = matches.opt_str("L") {
        log_file = Some(lfile);
    };
    if let Some(pfile) = matches.opt_str("P") {
        pid_file = Some(pfile);
    };
    //Parse configuration
    let octopipes_cfg: config::Config =
        match config::Config::parse_config(configuration_file.clone()) {
            Ok(cfg) => cfg,
            Err(error) => {
                panic!(
                    "Could not parse configuration file '{}': {} ({:?})",
                    configuration_file, error.message, error.code
                );
            }
        };
    //Override logger options
    if log_file.is_none() {
        log_file = Some(octopipes_cfg.log_config.log_file);
    };
    if log_level.is_none() {
        log_level = Some(octopipes_cfg.log_config.log_level);
    };
    let log_enabled: bool = match log_level.unwrap() {
        0 => false,
        _ => true,
    };
    //@! Initialize logger
    let log: logger::OctoLogger = logger::OctoLogger::new(
        log_enabled,
        logger::OctoLogLevel::from_int(log_level.unwrap()),
        String::from(log_file.as_ref().unwrap()),
        octopipes_cfg.log_config.stdout,
    );
    //Get PID and report program started
    let pid: u32 = std::process::id();
    log.info(format_args!(
        "octopipes-server [{}] started with PID {}",
        OCTOPIPES_SERVER_VERSION, pid
    ));
    //Dump configuration
    log.debug(format_args!("Logging configuration"));
    log.debug(format_args!("log-enabled: {}", log_enabled));
    log.debug(format_args!("log-level: {}", log_level.unwrap()));
    log.debug(format_args!("log-file: {}", log_file.unwrap()));
    log.debug(format_args!(
        "log-stdout: {}",
        octopipes_cfg.log_config.stdout
    ));
    log.debug(format_args!("Pipes configuration"));
    log.debug(format_args!("cap-pipe: {}", cap_path.as_ref().unwrap()));
    log.debug(format_args!("client-dir: {}", client_dir.as_ref().unwrap()));
    log.debug(format_args!("Protocol configuration"));
    log.debug(format_args!(
        "protocol_version: {}",
        octopipes_cfg.protocol_config.version
    ));
    //@! Write PID file
    if pid_file.is_some() {
        if let Err(err) = write_pid(pid_file.unwrap(), pid) {
            log.error(format_args!("Could not write PID to file: {}", err));
        };
    };
    //@! Start SIGINT listener
    let (tx_channel, rx_channel) = mpsc::channel();
    ctrlc::set_handler(move || {
        if let Err(..) = tx_channel.send(1) {
            panic!("Could not send CTRL-C");
        }
    })
    .expect("Error setting Ctrl-C handler");
    //@! Initialize OctopipesServer
    let protocol_version: OctopipesProtocolVersion =
        int_to_protocol_version(octopipes_cfg.protocol_config.version);
    let mut octopipes_server: OctopipesServer =
        OctopipesServer::new(protocol_version, cap_path.unwrap(), client_dir.unwrap());
    log.debug(format_args!("Initialized Octopipes Server"));
    //Start server
    if let Err(err) = octopipes_server.start_cap_listener() {
        log.error(format_args!("Could not start octopipes server: {}", err));
        std::process::exit(1);
    }
    log.info(format_args!("Octopipes Server CAP listener started"));
    //@!Main loop
    loop {
        //@! Process CAP message
        match octopipes_server.process_cap_once() {
            Ok(requests) => {
                if requests > 0 {
                    log.info(format_args!("Served {} requests on the CAP", requests));
                    let clients: Vec<String> = octopipes_server.get_clients();
                    for client in clients {
                        log.info(format_args!(
                            "Client '{}' is subscribed to {:?}",
                            client,
                            octopipes_server.get_subscriptions(client.clone())
                        ));
                    }
                }
            }
            Err(error) => {
                log.warn(format_args!("Could not serve request on CAP: {}", error));
            }
        };
        //@! Process inbox
        match octopipes_server.process_once() {
            Ok(requests) => {
                if requests > 0 {
                    log.info(format_args!("Processed {} requests from clients", requests));
                }
            }
            Err((client, error)) => {
                log.warn(format_args!(
                    "Could not process request from {}: {}",
                    client, error
                ));
            }
        };
        //@! Check SIGINT
        match rx_channel.try_recv() {
            Ok(..) => {
                break;
            }
            Err(error) => match error {
                mpsc::TryRecvError::Empty => {
                    //Continue
                }
                _ => {
                    panic!("CTRL+C thread is dead");
                }
            },
        };
        //Sleep 50ms
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    //@! Stop server
    if let Err(err) = octopipes_server.stop_server() {
        log.error(format_args!("Could not stop octopipes server: {}", err));
    }
    //@! Exit with RC 0
    std::process::exit(0);
}
