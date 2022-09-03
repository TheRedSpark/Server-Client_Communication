use std::io::Write;

fn main() {
    let mut conn = std::net::TcpStream::connect("localhost:8000").unwrap();

    conn.write(b"Hello Server").unwrap();
    conn.flush().unwrap();
}