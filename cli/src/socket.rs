use std::{net::{UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr}, time::Duration};

use utility::Blob;

pub enum DNSSocket {
    GOOGLE,
    CLOUDFLARE,
}

impl DNSSocket {
    fn get_ip(&self) -> SocketAddr {
        match self {
            DNSSocket::GOOGLE => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(8,8,4,4), 53)),
            DNSSocket::CLOUDFLARE => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1,1,1,1), 53)),
        }
    }
}

pub struct UDPClient{}
impl UDPClient {

    pub fn send_and_recieve(&self, msg: Vec<u8>, server: DNSSocket) -> Result<Vec<u8>, String> {
        let msg_bytes = msg.to_socket_msg().expect("Failed to parse msg to socket data.");
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).expect("Failed to initialize udp socket.");
        println!("Socket bound to local address 0.0.0.0:0");
        socket.set_read_timeout(Some(Duration::from_secs(5))).expect("Failed to set socket read time out");
        socket.connect(server.get_ip()).expect("Failed to connect to DNS socket.");
        let mut buf  = vec![0;4096];
        let send_bytes = socket.send(&msg_bytes).expect("Failed to send message over socket.");
        println!("Send {} bytes over socket", send_bytes);
        match socket.recv(&mut buf) {
            Ok(recieved) => println!("Recieved {} bytes over socket", recieved),
            Err(e) => println!("Failed to read from socket: {e:?}"),
        };
        return Ok(buf);
    }
}
