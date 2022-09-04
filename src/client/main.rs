use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    use std::net::{Shutdown, TcpStream};

    let mut stream = TcpStream::connect("127.0.0.1:80")
        .expect("Couldn't connect to the server...");
    println!("Connectet");
    let mut buf = String::new();
    stream.write(b"Echo").unwrap();
    stream.read_to_string(&mut buf).unwrap();
    println!("{buf}");
    let msg = b"Echo";
    let mut data = [0 as u8; 4];
    match stream.read_exact(&mut data) {
        Ok(_) => {
            if &data == msg {
                println!("Reply is ok!");
            } else {
                let text = from_utf8(&data).unwrap();
                println!("Unexpected reply: {}", text);
            }
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    stream.read_to_string(&mut buf).unwrap();
    println!("Der Befehl lautet: {buf}");
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
}