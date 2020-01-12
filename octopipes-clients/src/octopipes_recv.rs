//! # Octopipes-Clients
//!
//! `octopipes-recv` provides a simple binary to listen to some groups for messages from an Octopipes Server.

//
//   Octopipes-Clients
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
extern crate rand;
extern crate rustypipes;

use getopts::Options;
use rand::{thread_rng, Rng};
use rustypipes::{OctopipesClient, OctopipesProtocolVersion};
use std::env;
use std::process::exit;
use std::sync::mpsc;

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options] GROUPS", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();
    let cap_path: String;
    let clid: String;
    let verbose: bool;
    let message_amount: u32;
    let mut groups: Vec<String>;
    let mut exit_code: i32 = 0;
    //Get opts
    let mut opts = Options::new();
    opts.optopt("c", "cap-path", "Specify CAP path", "<CAP_PATH>");
    opts.optopt(
        "a",
        "count",
        "Specify the amount of message to receive before terminating (if 0, won't terminate)",
        "<AMOUNT>",
    );
    opts.optopt("C", "clid", "Specify the client id", "<CLIENT_ID>");
    opts.optflag(
        "v",
        "verbose",
        "Verbose mode prints messages as {ORIGIN} {PAYLOAD}",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    match matches.opt_str("c") {
        Some(cap) => {
            cap_path = cap;
        }
        None => {
            println!("CAP path must be specified");
            print_usage(&program, opts);
            return;
        }
    };
    match matches.opt_str("a") {
        Some(amount_str) => {
            match amount_str.parse::<u32>() {
                Ok(amount) => {
                    message_amount = amount;
                }
                Err(..) => {
                    panic!("Message amount is NaN");
                }
            };
        }
        None => {
            message_amount = 0;
        }
    };
    if matches.opt_present("v") {
        verbose = true;
    } else {
        verbose = false;
    }
    match matches.opt_str("C") {
        Some(client_id) => {
            clid = client_id;
        }
        None => {
            //Generate a random client id
            let rng = thread_rng();
            clid = rng
                .sample_iter(rand::distributions::Alphanumeric)
                .take(16)
                .collect::<String>();
        }
    };
    //Get groups
    if matches.free.is_empty() {
        println!("GROUPS must be specified");
        print_usage(&program, opts);
        return;
    } else {
        groups = Vec::with_capacity(matches.free.len());
        for grp in matches.free {
            groups.push(grp.clone());
        }
    }
    //Options OK!
    //Set CTRL+C handler
    let (tx_channel, rx_channel) = mpsc::channel();
    ctrlc::set_handler(move || {
        if let Err(..) = tx_channel.send(1) {
            panic!("Could not send CTRL-C");
        }
    })
    .expect("Error setting Ctrl-C handler");
    //Instance client now
    let mut client: OctopipesClient =
        OctopipesClient::new(clid, cap_path, OctopipesProtocolVersion::Version1);
    if let Err(error) = client.subscribe(&groups) {
        println!("Could not subscribe to Octopipes Server: {}", error);
        exit(1);
    }
    let mut current_message_count: u32 = 0;
    while current_message_count < message_amount && message_amount != 0 {
        match client.get_next_message() {
            Ok(msg_opt) => {
                match msg_opt {
                    Some(message) => {
                        //Convert data to string
                        let data_str: String = String::from_utf8(message.data).unwrap();
                        if verbose {
                            println!("{} > {}", message.origin.unwrap(), data_str);
                        } else {
                            println!("{}", data_str);
                        }
                        current_message_count += 1;
                    }
                    None => continue,
                };
            }
            Err(error) => {
                println!("Error while fetching inbox: {}", error);
                exit_code = 1;
                break;
            }
        };
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
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    //Unsubscribe
    if let Err(error) = client.unsubscribe() {
        println!("Could not unsubscribe from server: {}", error);
        exit(1);
    }
    //Exit
    exit(exit_code);
}
