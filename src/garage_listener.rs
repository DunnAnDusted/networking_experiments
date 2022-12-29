use std::{io, net::{UdpSocket, Ipv4Addr}};

pub fn listen() -> io::Result<()> {
    static  IP_MULTI: Ipv4Addr = Ipv4Addr::new(224, 192, 32, 29);
    let listener = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 22600))?;
    listener.join_multicast_v4(&IP_MULTI, &Ipv4Addr::UNSPECIFIED)?;

    let mut buff = [0; 256];

    loop {
        let size = loop {
            let size = listener.recv(&mut buff)?;
        
            match (size, buff) {
                (3, [71, 61, _x @ 48..=51, ..]) => break size, // Read expected data
                (0, _) => return Ok(()), // Socket has closed
                _ => continue, // Unexpected data
            }
                
        };
        
        let buff = String::from_utf8_lossy(&buff[..size]);
        
        println!("\x1B[2J\x1B[1;1HSize: {size:?}\nContents: {buff:?}");
    }
}