use clap::Parser as clapParser;
use parsing::{Parser, Query, Response};

pub mod socket;

#[derive(clapParser, Debug)]
#[clap(name = "askrs")]
#[clap(author = "philmish")]
#[clap(version = "0.1")]
#[clap(about = "CLI tool for requesting dns records.", long_about = None)]
struct Flags {

    /// Target adress or domain to request records for.
    #[clap(short, long)]
    target: String,

    /// DNS server to use for request.
    #[clap(short, long, default_value = "google")]
    server: String,

    /// Record type to request (A, AAAA, MX, CNAME)
    #[clap(short, long, default_value = "A")]
    record: String,
}

pub struct CLI {
    flags: Flags,
    parser: Parser,
}

impl CLI {

    pub fn init() -> Self {
        let flags = Flags::parse();
        let parser = Parser{};
        return Self{
            flags,
            parser
        };
    }

    fn send_query(&self, q: Query, srv: socket::DNSSocket) -> Vec<u8> {
        let client = socket::UDPClient{};
        let msg = q.to_bytes();
        let a = client.send_and_recieve(
            msg,
            srv
            )
            .unwrap();
        q.print();
        return a;
    }

    pub fn run(&self) {
        let q = self.parser.new_query(
            self.flags.target.clone(),
            self.flags.record.clone()
            );
        let a = self.send_query(q, socket::DNSSocket::from_string(&self.flags.server));
        println!("----------------------");
        let resp = Response::from_bytes(a, Parser{}).unwrap();
        resp.print();
    }
}
