use std::{
    io::{self, prelude::*},
    net::{Ipv4Addr, TcpListener},
    process::Command,
};

pub fn listen() -> io::Result<()> {
    let mut buff = [0; 1024];

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 7878))?;
    Command::new("curl")
        .arg("http://127.0.0.1:7878")
        .arg("--http0.9")
        .spawn()?;

    let (mut stream, ip) = listener.accept()?;

    let size = stream.read(&mut buff)?;
    let buff = String::from_utf8_lossy(&buff[..size]);

    println!("IP: {ip:?}\nSize: {size:?}\nContents: {buff:?}");

    const RESPONSE: &[u8] =
        b"HTTPS/1.1 405 Method Not Allowed\r\nServer: The-Rust-Machine\r\nAllow:\r\n";

    stream.write(RESPONSE)?;
    stream.flush()
}
