//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#![allow(dead_code)]

extern crate static_assertions as sa;

use crate::client::launch_client;
use crate::server::launch_server;
use std::env;
use std::io::{Error, ErrorKind};

mod attacker;
mod client;
mod config;
mod defender;
mod game_engine;
mod game_interface;
mod player;
mod server;
mod ship;
mod types;

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage:\n[program.exe] [server|client] [Address and Port]");
        return Err(Error::new(ErrorKind::InvalidData, "Invalid arguments"));
    }

    if &args[1] == "server" {
        match launch_server(args[2].to_owned()) {
            Ok(()) => {}
            Err(err) => eprintln!("{}", err),
        }
    } else if &args[1] == "client" {
        match launch_client(args[2].to_owned()) {
            Ok(()) => {}
            Err(err) => eprintln!("{}", err),
        }
    } else {
        eprintln!("Usage:\n[program.exe] [server|client] [Address and Port]");
        return Err(Error::new(ErrorKind::InvalidData, "Invalid argument"));
    }

    Ok(())
}
