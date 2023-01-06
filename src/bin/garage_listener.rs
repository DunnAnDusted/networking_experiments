use std::{
    io,
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

pub fn main() -> io::Result<()> {
    static IP_MULTI: Ipv4Addr = Ipv4Addr::new(224, 192, 32, 29);
    const TIMEOUT: Duration = Duration::from_secs(2);

    let mut buff = [0; 256];
    let listener = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 22600))?;

    listener.join_multicast_v4(&IP_MULTI, &Ipv4Addr::UNSPECIFIED)?;
    listener.set_read_timeout(Some(TIMEOUT))?;

    loop {
        let (size, addr) = loop {
            let (size, addr) = listener.recv_from(&mut buff)?;

            match (size, buff) {
                (3, [71, 61, _x @ 48..=51, ..]) => break (size, addr), // Read expected data
                (0, _) => return Ok(()),                               // Socket has closed
                _ => continue,                                         // Unexpected data
            }
        };

        let buff = String::from_utf8_lossy(&buff[..size]);

        println!(
            "\x1B[2J\x1B[1;1H\
            Size: {size:?}\n\
            Contents: {buff:?}\n\
            Address: {addr:?}"
        );
    }
}
