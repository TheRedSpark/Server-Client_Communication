use std::net::{TcpStream};
use std::io::{Read, Write};
//use std::simd::u8x16;
use std::str::from_utf8;
use std::str;

fn main() {
    use std::net::{Shutdown, TcpStream};
    let mut buf = [0 as u8; 32];
    let client_id = b"c0000001";
    let echo = b"jejdkztb";
    let data = [0 as u8; 16];
    let mut command = [0 as u8; 16];
    let mut i = 0;
    while i < 30 {
        let mut stream = TcpStream::connect("127.0.0.1:80")
            .expect("Couldn't connect to the server...");
        println!("Connectet");
        stream.write(echo).unwrap();
        stream.read(&mut buf).unwrap();
        if &buf[0..8] == echo {
            println!("Echo complete");
            stream.write(client_id).unwrap();
        }
        i += 1;
        println!("{}",i);
        stream.read(&mut buf).unwrap();
        println!("Der Befehl lautet: {}",str::from_utf8(&buf).unwrap());
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }

}