#![deny(missing_docs)]
//!
//! SOCKS5 implementation in RUST
//!

#[macro_use] extern crate log;
#[macro_use] extern crate clap;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Error};


/// initialize the logging subsystem
/// warn,error to the console
/// info and above to the file
fn init_logger() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("logs.log").unwrap()),
        ]
    ).unwrap();
}

/// setup the cli parsing app
fn init_clapp() -> clap::ArgMatches<'static> {
    let matches = clap_app!(myapp => 
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        

    ).get_matches();
    return matches;
}

fn handle_client(mut stream: TcpStream) {
    let mut oneb: [u8;1] = [b0];
    match stream.read_exact(&oneb) {
        Ok(()) => {
            // pass
        }
        Err(e) => {

        }
    }

    if oneb[0] != 0x5 {
        error!("Client version is not 5 - closing. (was {})", oneb[0]);
        match stream.shutdown(Shutdown::Both) {
            Ok(()) => {
                return;
            }
            Err(E) => {
                error!("Error shutting down stream, ({:?})", E);
            }
        }
    }

    stream.read_ex

}

/// listen and loop over all connections
fn handle_loop() {
    let listener = TcpListener::bind("0.0.0.0:1080").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);       
            }
            Err(e) => {
                error!("Server error: {:?}", e)
            }
        }
    }
}

/// run the code!
fn main() {
    init_logger();
    init_clapp();

    handle_loop();
}
