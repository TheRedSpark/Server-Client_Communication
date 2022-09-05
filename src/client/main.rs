use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::str;

fn main() {
    use std::net::{Shutdown, TcpStream};

    let mut stream = TcpStream::connect("127.0.0.1:80")
        .expect("Couldn't connect to the server...");
    println!("Connectet");
    let mut buf = [0 as u8; 4];
    let echo = b"Echo";
    stream.write(echo).unwrap();
    stream.read(&mut buf).unwrap();
    if &buf == echo {
        println!("Echo complete")
    }
    println!("Der Befehl lautet: {}",str::from_utf8(&buf).unwrap());
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
}