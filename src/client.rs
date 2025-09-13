use std::net::{Ipv4Addr, SocketAddrV4};

use crate::dns::{Message, Question, header::Header};

pub enum DnsServer {
    GOOGLE,
    CLOUDFLARE,
    QUAD9,
}

impl Into<SocketAddrV4> for DnsServer {
    fn into(self) -> SocketAddrV4 {
        match self {
            DnsServer::GOOGLE => SocketAddrV4::new(Ipv4Addr::new(8, 8, 8, 8), 53),
            DnsServer::QUAD9 => SocketAddrV4::new(Ipv4Addr::new(9, 9, 9, 9), 53),
            DnsServer::CLOUDFLARE => SocketAddrV4::new(Ipv4Addr::new(1, 1, 1, 1), 53),
        }
    }
}

pub struct DnsClient<const BUF_SIZE: usize = 4096> {
    buf: [u8; BUF_SIZE],
}

impl<const BUF_SIZE: usize> DnsClient<BUF_SIZE> {
    pub fn query(&mut self, h: Header, q: Question, srv: DnsServer) -> Response {
        let mut len = 0usize;
        len += h.copy_bytes(&mut self.buf[0..12]);
        Err(ClientError::NoResponse)
    }

    pub fn inverse_query_ipv4(&mut self, h: Header, addr: Ipv4Addr, srv: DnsServer) -> Response {
        Err(ClientError::NoResponse)
    }
}

type Response = Result<Message, ClientError>;

pub enum ClientError {
    NoResponse,
}
