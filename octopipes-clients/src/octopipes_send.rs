//! # Octopipes-Clients
//!
//! `octopipes-send` provides a simple binary to send a message to a certain group through an Octopipes Server.

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

extern crate getopts;
extern crate rand;
extern crate rustypipes;

use getopts::Options;
use rand::{thread_rng, Rng};
use rustypipes::{OctopipesClient, OctopipesProtocolVersion};
use std::env;
use std::process::exit;

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();
    let cap_path: String;
    let clid: String;
    let payload: String;
    let remote: String;
    let mut exit_code: i32 = 0;
    //Get opts
    let mut opts = Options::new();
    opts.optopt("c", "cap-path", "Specify CAP path", "<CAP_PATH>");
    opts.optopt("r", "remote", "Specify the remote", "<REMOTE>");
    opts.optopt("p", "payload", "Specify the payload to send", "<PAYLOAD>");
    opts.optopt("C", "clid", "Specify the client id", "<CLIENT_ID>");
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
    match matches.opt_str("r") {
        Some(remote_group) => {
            remote = remote_group;
        }
        None => {
            println!("remote must be specified");
            print_usage(&program, opts);
            return;
        }
    };
    match matches.opt_str("p") {
        Some(data) => {
            payload = data;
        }
        None => {
            println!("payload must be specified");
            print_usage(&program, opts);
            return;
        }
    };
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
    //Options OK!
    //Instance client now
    let mut client: OctopipesClient =
        OctopipesClient::new(clid, cap_path, OctopipesProtocolVersion::Version1);
    if let Err(error) = client.subscribe(&vec![]) {
        println!("Could not subscribe to Octopipes Server: {}", error);
        exit(1);
    }
    //Send data
    let mut data: Vec<u8> = Vec::with_capacity(payload.len());
    for ch in payload.as_bytes() {
        data.push(*ch as u8);
    }
    if let Err(error) = client.send(&remote, data) {
        println!("Could not send data to {}: {}", remote, error);
        exit_code = 1;
    }
    //Unsubscribe
    if let Err(error) = client.unsubscribe() {
        println!("Could not unsubscribe from server: {}", error);
        exit(1);
    }
    //Exit
    exit(exit_code);
}
