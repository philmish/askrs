use clap::ArgAction;
use clap::Parser as clapParser;
use dns::record::RecordType;
use parsing::byte_stream_parser::ByteStreamParser;
use parsing::Query;
use socket::DNSSocket;

pub mod socket;

#[derive(clapParser, Debug)]
#[clap(name = "askrs")]
#[clap(author = "philmish")]
#[clap(version = "0.1")]
#[clap(about = "CLI tool for requesting dns records.", long_about = None)]
struct Flags {
    /// Target adress or domain to request records for.
    #[clap(short, long)]
    uri: String,

    /// DNS server to use for request.
    #[clap(short, long, default_value = "google")]
    server: String,

    /// Record type to request (A, AAAA, MX, CNAME)
    #[clap(long, default_value = "A")]
    record: String,

    /// Recursive Query
    #[clap(short = 'r', long = "recursion_desired", action = ArgAction::SetTrue)]
    rd: bool,

    /// Verbose Output
    #[clap(short = 'v', long = "verbose", action = ArgAction::SetTrue)]
    verbose: bool,
}

impl Flags {
    pub fn get_rtype(&self) -> RecordType {
        RecordType::from_string(self.record.clone())
    }

    pub fn get_server(&self) -> DNSSocket {
        DNSSocket::from_string(&self.server)
    }
}

pub struct CLI {
    flags: Flags,
}

impl CLI {
    pub fn init() -> Self {
        let flags = Flags::parse();
        Self { flags }
    }

    fn send_query(&self, q: Query, srv: DNSSocket, verbose: bool) -> Vec<u8> {
        let client = socket::UDPClient {};
        let msg = q.to_bytes();
        let a = client.send_and_recieve(msg, srv).unwrap();
        if verbose {
            q.print(verbose);
        }
        a
    }

    pub fn run(&self) {
        let qry = Query::new(
            self.flags.uri.clone(),
            self.flags.get_rtype(),
            self.flags.rd,
        );
        let a = self.send_query(qry, self.flags.get_server(), self.flags.verbose);
        let resp = ByteStreamParser::new(&a).parse_response().unwrap();
        resp.print(self.flags.verbose);
    }
}
