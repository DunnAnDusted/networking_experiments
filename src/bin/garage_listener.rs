use std::{
    io, iter,
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

pub fn main() -> io::Result<()> {
    static IP_MULTI: Ipv4Addr = Ipv4Addr::new(224, 192, 32, 29);
    const TIMEOUT: Duration = Duration::from_secs(2);

    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 22600))?;

    sock.join_multicast_v4(&IP_MULTI, &Ipv4Addr::UNSPECIFIED)?;
    sock.set_read_timeout(Some(TIMEOUT))?;

    iter::from_fn(|| {
        let mut buff = [0; 256];

        sock.recv_from(&mut buff)
            .map(|(size, addr)| size.ne(&0).then_some((size, addr, buff)))
            .transpose()
    })
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
