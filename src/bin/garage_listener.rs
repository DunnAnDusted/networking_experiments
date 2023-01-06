use std::{
    io, iter,
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

pub fn main() -> io::Result<()> {
    static IP_MULTI: Ipv4Addr = Ipv4Addr::new(224, 192, 32, 29);
    const TIMEOUT: Duration = Duration::from_secs(2);

    let listener = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 22600))?;

    listener.join_multicast_v4(&IP_MULTI, &Ipv4Addr::UNSPECIFIED)?;
    listener.set_read_timeout(Some(TIMEOUT))?;

    iter::repeat_with(|| {
        let mut buff = [0; 256];

        listener
            .recv_from(&mut buff)
            .map(|(size, addr)| (size, addr, buff))
    })
    .take_while(|x| !matches!(x, Ok((0, ..))))
    .filter(|x| matches!(x, Err(_) | Ok((3, _, [71, 61, 48..=51, ..]))))
    .try_for_each(|x| {
        x.map(|(size, addr, buff)| {
            let buff = String::from_utf8_lossy(&buff[..size]);

            println!(
                "\x1B[2J\x1B[1;1H\
                Size: {size:?}\n\
                Contents: {buff:?}\n\
                Address: {addr:?}"
            );
        })
    })
}
