use std::{
    fmt::Display,
    io,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::Duration,
};

use crate::{
    dns::{
        error::DnsError,
        header::{HeaderReadWriter, MessageHeader, Opcode},
        message::Message,
        question::Question,
    },
    ressources::DnsServer,
};

const DEFAULT_CLIENT_BUF_SIZE: usize = 4096;

pub struct DnsClient<const BUF_SIZE: usize> {
    buf: [u8; BUF_SIZE],
    cursor: usize,
    hrw: HeaderReadWriter,
}

impl<const BUF_SIZE: usize> DnsClient<BUF_SIZE> {
    const BIND_ADDR: &str = "0.0.0.0:0";

    pub fn new() -> Self {
        Self {
            buf: [0u8; BUF_SIZE],
            cursor: 0,
            hrw: HeaderReadWriter::new(),
        }
    }

    pub fn standard_query(&mut self, q: Question, srv: &DnsServer) -> Response<'_> {
        let mut header = MessageHeader::empty();
        self.hrw.write_id(0x0303, &mut header);
        self.hrw.write_opcode(Opcode::StandardQuery, &mut header);
        self.hrw.write_recursion_desired(&mut header);
        self.hrw.write_qd_count(1, &mut header);
        header.copy_into(&mut self.buf[0..12]);
        let q_btyes = q.try_to_bytes()?;
        let query_len = q_btyes.len() + 12;
        self.buf[12..query_len].copy_from_slice(q_btyes.as_slice());
        let socket = UdpSocket::bind(Self::BIND_ADDR)?;
        socket.set_read_timeout(Some(Duration::from_secs(3)))?;
        socket.connect(SocketAddr::from(srv))?;
        let _ = socket.send(&self.buf[0..query_len])?;
        let rcvd = socket.recv(&mut self.buf[0..512])?;
        let resp_header = MessageHeader::try_from(&self.buf[0..12])?;
        Ok(&self.buf[0..rcvd])
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

pub fn default_dns_client() -> DnsClient<DEFAULT_CLIENT_BUF_SIZE> {
    DnsClient::new()
}

#[derive(Debug)]
pub enum ClientError {
    NoResponse,
    ConnectionError(io::Error),
    ParsingError(DnsError),
}

type Response<'a> = Result<&'a [u8], ClientError>;

impl std::error::Error for ClientError {}

impl From<io::Error> for ClientError {
    fn from(value: io::Error) -> Self {
        Self::ConnectionError(value)
    }
}

impl<T> From<T> for ClientError
where
    T: Into<DnsError>,
{
    fn from(value: T) -> Self {
        Self::ParsingError(value.into())
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
