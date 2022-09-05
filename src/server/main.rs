use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    println!("Client connectet");
    let mut data = [0 as u8; 50];
    stream.read(&mut data).unwrap();
    stream.write(&data).unwrap();
    //stream.write(b"Herunterfahren").unwrap();
    println!("Echo send wait for Reply")
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
