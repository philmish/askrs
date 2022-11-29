use std::net::UdpSocket;

use utility::Blob;

pub enum DNSSocket {
    GOOGLE,
}

pub struct UDPClient {
    socket: UdpSocket,
}

impl UDPClient {
    pub fn new(server: DNSSocket) -> Self {
        let socket: UdpSocket = match server {
            DNSSocket::GOOGLE => UdpSocket::bind("8.8.8.8:53").expect("Failed to bind socket to google DNS."),
        };
        return UDPClient{
            socket
        };
    }

    pub fn send_and_recieve(&self, msg: Vec<u8>) -> Result<Vec<u8>, String> {
        let msg_bytes = msg.to_socket_msg().expect("Failed to parse msg to socket data.");
        let mut buf = [0;4096];
        let send_bytes = self.socket.send(msg_bytes).expect("Failed to send message over socket.");
        println!("Send {} bytes over socket", send_bytes);
        let response = self.socket.recv(&mut buf).expect("Failed to read response from socket.");
        println!("Recieved {} bytes over socket", response);
        return Ok(buf.to_vec());
    }
}
