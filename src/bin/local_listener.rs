use std::{
    io::{self, prelude::*},
    net::{Ipv4Addr, TcpListener},
    process::Command,
};

pub fn main() -> io::Result<()> {
    let mut buff = [0; 1024];
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 7878))?;

    Command::new("curl")
        .arg("http://127.0.0.1:7878")
        .arg("--http0.9")
        .spawn()?;

    let (mut stream, ip) = listener.accept()?;

    let size = stream.read(&mut buff)?;
    let buff = String::from_utf8_lossy(&buff[..size]);

    println!(
        "IP: {ip:?}\n\
        Size: {size:?}\n\
        Contents: {buff:?}"
    );

    const RESPONSE: &[u8] = b"HTTP/1.1 405 Method Not Allowed\r\n\
        Allow:\r\n\
        Content-Length: 0\r\n\r\n";

    stream.write(RESPONSE)?;
    stream.flush()
}
