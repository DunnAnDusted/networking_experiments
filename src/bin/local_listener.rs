use std::{
    io::{self, prelude::*},
    net::{Ipv4Addr, TcpListener},
    process::Command,
};

pub fn main() -> io::Result<()> {
    let mut buff = [0; 1024];
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 7878))?;

    // Runs `cURL` as a  child process, to ensure consistent arguments,
    // by avoiding the need to execute it manually from another shell tab.
    let mut child = Command::new("curl")
        .arg("http://127.0.0.1:7878")
        .arg("-i")
        .spawn()?;


    let (mut stream, ip) = listener.accept()?;
    let size = stream.read(&mut buff)?;
    let buff = String::from_utf8_lossy(&buff[..size]);

    println!(
        "IP: {ip}\n\
        Size: {size}\n\
        Contents: {buff:?}\n"
    );

    // Hard-coded response, because the program
    // is designed to explore the structure of `cURL` HTTP requests,
    // rather than provide any legitimate responses.
    const RESPONSE: &[u8] = b"HTTP/1.1 405 Method Not Allowed\r\n\
        Allow:\r\n\
        Content-Length: 0\r\n\r\n";

    stream.write(RESPONSE)?;
    stream.flush()?;

    // Waits on `cURL` to exit,
    // to ensure its output can be viewed.
    child.wait().map(drop)
}
