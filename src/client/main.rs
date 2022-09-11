#![allow(unused)]

use std::net;
use std::net::{TcpStream, AddrParseError};
use std::io::{Empty, Error, Read, Write};
//use std::simd::u8x16;
use std::str;
use std::{thread, time};
use std::fmt::Error as OtherError;

fn communication() -> Result<_, _> {
    use std::net::{Shutdown, TcpStream};
    let mut buf = [0 as u8; 48];
    let client_id = b"c0000001";
    let echo_key = b"jejdkztb";
    let data = b"D000000000000003";
    let command = b"C000000000000001";
    let mut stream = TcpStream::connect("127.0.0.1:80")
        .expect("Couldn't connect to the server...");
    println!("Connectet");
    stream.write(echo_key).unwrap();
    stream.write(client_id).unwrap();
    stream.write(data).unwrap();
    stream.write(command).unwrap();
    stream.read(&mut buf).unwrap();
    println!("Der Befehl lautet: {}", str::from_utf8(&buf).unwrap());
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    thread::sleep(time::Duration::from_millis(5000));
    let s = "Hat geklappt";
    Ok()
}

fn main() {
    let mut i = 0;
    while i < 5 {
        match communication() {
            Ok(t) => println!("We've time travelled to !!"),
            Err(..) => eprintln!("Oh noes, we don't know which era we're in! :( \n "),
        }
        i = i + 1;
    }
}