use std::{collections::HashMap, io, net::Ipv4Addr};

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::State> = Default::default();
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        eprint!("{}", connections.len());
        let nbytes = nic.recv(&mut buf[..])?;
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // not IPv4
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip) => {
                let src = ip.source_addr();
                let dst = ip.destination_addr();
                if ip.protocol() != 0x06 {
                    // not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip.slice().len()..nbytes]) {
                    Ok(tcp) => {
                        let datai = 4 + ip.slice().len() + tcp.slice().len();
                        connections
                            .entry(Quad {
                                src: (src, tcp.source_port()),
                                dst: (dst, tcp.destination_port()),
                            })
                            .or_default()
                            .on_packet(ip, tcp, &buf[datai..nbytes]);
                    }
                    Err(e) => {
                        eprint!("Ignored tcp packet {:?}", e);
                    }
                };
            }
            Err(e) => {
                eprint!("Ignored packet {:?}", e);
            }
        }
    }
}
