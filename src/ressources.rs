use std::net::SocketAddr;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum DnsServer {
    GOOGLE,
    CLOUDFLARE,
    QUAD9,
    Custom(SocketAddr),
}

impl From<&DnsServer> for SocketAddr {
    fn from(value: &DnsServer) -> Self {
        match value {
            DnsServer::CLOUDFLARE => SocketAddr::from(([1, 1, 1, 1], 53)),
            DnsServer::GOOGLE => SocketAddr::from(([8, 8, 8, 8], 53)),
            DnsServer::QUAD9 => SocketAddr::from(([1, 1, 1, 1], 53)),
            DnsServer::Custom(a) => *a,
        }
    }
}
