use std::env;
use std::net::{UdpSocket, Ipv4Addr};

struct RtpPacket<'buf> {
    buf: &'buf [u8],
    size: usize,
}

impl<'buf> RtpPacket<'buf> {
    pub fn new(buf: &'buf [u8], len: usize) -> RtpPacket<'buf> {
        assert!(!buf.is_empty());
        RtpPacket {
            buf: buf,
            size: len
        }
    }
    pub fn length(&self) -> usize {
        self.size
    }
    pub fn ver(&self) -> u8 {
        (self.buf[0] & 0xc0) >> 6
    }

    pub fn padding(&self) -> bool {
        (self.buf[0] & 0x20) != 0
    }

    pub fn extension(&self) -> bool {
        (self.buf[0] & 0x10) != 0
    }

    pub fn csrc_count(&self) -> u8 {
        self.buf[0] & 0x0f
    }

    pub fn maker(&self) -> bool {
        (self.buf[1] & 0x80) != 0
    }

    pub fn payload_type(&self) -> u8 {
        self.buf[1] & 0x7f
    }

    pub fn seq(&self) -> u16 {
        (self.buf[2] as u16) << 8 | self.buf[3] as u16
    }

    pub fn timestamp(&self) -> u32 {
        (self.buf[4] as u32) << 24 |
            (self.buf[5] as u32) << 16 |
            (self.buf[6] as u32) << 8 |
            (self.buf[7] as u32)
    }

    pub fn ssrc(&self) -> u32 {
        (self.buf[8] as u32) << 24 |
            (self.buf[9] as u32) << 16 |
            (self.buf[10] as u32) << 8 |
            (self.buf[11] as u32)
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);

    let bind_addr:Ipv4Addr  = args[1].parse().unwrap();
    let mcast_group: Ipv4Addr = args[2].parse().unwrap();
    let port: u16 = args[3].parse().unwrap();

    println!("Group:{} Port:{}", mcast_group, port);

    let socket = UdpSocket::bind(format!("{}:{}", bind_addr, port))
        .expect("Could not bind client socket");

    socket.join_multicast_v4(&mcast_group, &bind_addr)
        .expect("Could not join multicast group");

    let mut buffer = [0; 1500];
    loop {
        let (len, _) = socket.recv_from(&mut buffer)
            .expect("Failed to recv_from to server");

        let w = RtpPacket::new(&buffer, len);
        println!("len:{} ver:{} p:{}  e:{} cc:{} M:{} PT:{} seq:{:04x} timestamp:{} ssrc:{:x}",
                 w.length(), w.ver(), w.padding(), w.extension(), w.csrc_count(), w.maker(), w.payload_type(), w.seq(), w.timestamp(), w.ssrc());
    }
}

