use std::{
    fmt::Display,
    io,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
};

use crate::{
    dns::{error::DnsError, header::MessageHeader, message::Message, question::Question},
    ressources::DnsServer,
};

pub struct DnsClient<const BUF_SIZE: usize = 4096> {
    buf: [u8; BUF_SIZE],
    cursor: usize,
}

impl<const BUF_SIZE: usize> DnsClient<BUF_SIZE> {
    #[allow(dead_code)]
    pub fn query(&mut self, q: Question, srv: &DnsServer) -> Response<'_> {
        // let mut h = Header::new();
        // h.set_qd_count(1);
        // let mut len = 0usize;
        // len += h.copy_bytes(&mut self.buf[0..12]);
        // let socket = UdpSocket::bind(SocketAddr::from(srv))?;
        // let  = socket.send(&self.buf[0..len])?;
        // let received = socket.recv(&mut self.buf[0..])?;
        // if received == 0 {
        //     return Err(ClientError::NoResponse);
        // }
        // let resp = &self.buf[0..received];
        todo!()
    }

    pub fn inverse_query_ipv4(
        &mut self,
        h: MessageHeader,
        addr: Ipv4Addr,
        srv: &DnsServer,
    ) -> Response<'_> {
        todo!()
    }
}

const UDP_BUF_SIZE: usize = 512;

pub fn new_udp_client() -> DnsClient<UDP_BUF_SIZE> {
    return DnsClient {
        buf: [0u8; UDP_BUF_SIZE],
        cursor: 0,
    };
}

type Response<'a> = Result<Message<'a>, ClientError>;

impl From<io::Error> for ClientError {
    fn from(value: io::Error) -> Self {
        Self::ConnectionError(value)
    }
}

#[derive(Debug)]
pub enum ClientError {
    NoResponse,
    ConnectionError(io::Error),
    ParsingError(DnsError),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ClientError {}
