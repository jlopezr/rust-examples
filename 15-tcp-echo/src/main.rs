#[macro_use]
extern crate log;
extern crate env_logger;

use std::io;
use std::thread;
use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;

fn main() {
    env_logger::init().unwrap();

    let host = "127.0.0.1";
    let port = 8080;

    info!("Starting...");
    let sock = TcpListener::bind((host, port)).unwrap();

    for stream in sock.incoming() {
        match stream {
            Err(e) => warn!("Accept err {}", e),
            Ok(stream) => {
                thread::spawn(|| {
                    debug!("hanndle_client ended {:?}", handle_client(stream));
                });
            }
        }
    }
}

fn to_hex_string(ba: &[u8]) -> String {
    let strs: Vec<String> = ba.iter()
        .map(|b| format!("{:02X} ", b))
        .collect();
    strs.join("")
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    info!("New client {}", stream.peer_addr().unwrap());
    let mut buf: [u8; 4096] = [0; 4096];
    loop {
        debug!("waiting data from the client");
        let got = try!(stream.read(&mut buf[..]));
        debug!("got {} bytes", got);
        debug!("buf {}", to_hex_string(&buf[0..got]));
        if got == 0 {
            // Is it possible? Or IoError will be raised anyway?
            break
        }
        debug!("sending data back to the client");
        try!(stream.write(&buf[0..got]));
    }
    Ok(())
}
