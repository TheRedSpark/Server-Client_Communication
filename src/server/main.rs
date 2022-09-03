use std::io::Read;

fn main() {
    let listener = std::net::TcpListener::bind("localhost:8000").unwrap();
    let mut conn = listener.accept().unwrap();
    let mut buf = String::new();
    conn.0.read_to_string(&mut buf).unwrap();
    println!("{buf}");
}