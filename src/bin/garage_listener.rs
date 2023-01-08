use std::{
    io, iter,
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

pub fn main() -> io::Result<()> {
    static MULTICAST_ADDRESS: Ipv4Addr = Ipv4Addr::new(224, 192, 32, 29);
    const SOCKET_PORT: u16 = 22600;
    // Status should be broadcast roughly every second,
    // so a duration of two seconds, gives a bit of required leeway.
    const TIMEOUT: Duration = Duration::from_secs(2);
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, SOCKET_PORT))?;

    // Signel is broadcast over a UDP Multicast group,
    // so socket needs to join group via its address to recieve it.
    sock.join_multicast_v4(&MULTICAST_ADDRESS, &Ipv4Addr::UNSPECIFIED)?;
    sock.set_read_timeout(Some(TIMEOUT))?;

    iter::from_fn(|| {
        let mut buff = [0; 256];

        sock.recv_from(&mut buff)
            // If `size` is 0, the buffer has closed,
            // and the iterator should stop.
            .map(|(size, addr)| size.ne(&0).then_some((size, addr, buff)))
            .transpose()
    })
    // Drop items with unexpected data.
    .filter(|x| matches!(x, Err(_) | Ok((3, _, [71, 61, 48..=51, ..]))))
    .try_for_each(|x| {
        x.map(|(size, addr, buff)| {
            // Sucessfully read bytes, should read
            // "G=0..3", and can be converted to a string.
            //
            // This should always suceed, due to the iterator filtering,
            // but this handles instances where this isn't the case,
            // by indicating the location of invalid byte sequences,
            // avoiding heap allocation by default, and more detailed error handling.
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
