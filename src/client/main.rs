use std::net::{TcpStream};
use std::io::{Read, Write};
//use std::simd::u8x16;
use std::str;

fn main() {
    use std::net::{Shutdown, TcpStream};
    let mut buf = [0 as u8; 48];
    let client_id = b"c0000001";
    let echo_key = b"jejdkztb";
    //let mut data = [0 as u8; 16];
    //data = *b"0000000000000000";
    let data= b"D000000000000003";
    //let mut command = [0 as u8; 16];
    //command = *b"0000000000000000";
    let command= b"C000000000000001";
    let mut i = 0;
    while i < 1 {
        let mut stream = TcpStream::connect("127.0.0.1:80")
            .expect("Couldn't connect to the server...");
        println!("Connectet");
        stream.write(echo_key).unwrap();
        stream.write(client_id).unwrap();
        stream.write(data).unwrap();
        stream.write(command).unwrap();
        //stream.read(&mut buf).unwrap();
        /*if &buf[0..8] == echo_key {
            //println!("Echo complete");

        }*/
        i += 1;
        //println!("{}",i);
        stream.read(&mut buf).unwrap();
        println!("Der Befehl lautet: {}",str::from_utf8(&buf).unwrap());
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }

}