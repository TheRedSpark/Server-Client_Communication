use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut buff = [0 as u8; 48];
    stream.read(&mut buff).unwrap();
    let echo = str::from_utf8(&buff[0..8]).unwrap();
    let client_id = str::from_utf8(&buff[8..16]).unwrap();
    let data = str::from_utf8(&buff[16..32]).unwrap();
    let command = str::from_utf8(&buff[32..48]).unwrap();
    println!("Echo is :{}", echo);
    println!("Client {} ist verbunden", client_id);
    println!("Data is: {}", data);
    println!("Command is: {}", command);


    stream.write((&buff).as_ref()).unwrap();
    //stream.write(b"Herunterfahren").unwrap();
    //println!("Echo send wait for Reply")
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
