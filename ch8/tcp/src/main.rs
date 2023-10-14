use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let host = "example.com:80";
    let mut stream = std::net::TcpStream::connect(host)?;

    let _ = stream.write(b"GET / HTTP/1.0\r\n\r\n")?;
    let _ = stream.write(b"Host: www.rustinacion.com\r\n\r\n")?;

    let mut reader = BufReader::new(&stream);
    let buffer = reader.fill_buf()?.to_vec();
    println!("{}", String::from_utf8_lossy(&buffer));
    reader.consume(buffer.len());

    Ok(())
}
