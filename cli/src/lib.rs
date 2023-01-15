use clap::Parser as clapParser;
use clap::ArgAction;
use dns::record::RecordType;
use parsing::byte_stream_parser::ByteStreamParser;
use parsing::Query;

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

pub struct CLI {
    flags: Flags,
}

impl CLI {

    pub fn init() -> Self {
        let flags = Flags::parse();
        return Self{
            flags,
        };
    }

    fn send_query(&self, q: Query, srv: socket::DNSSocket, verbose: bool) -> Vec<u8> {
        let client = socket::UDPClient{};
        let msg = q.to_bytes();
        let a = client.send_and_recieve(msg, srv)
                      .unwrap();
        if verbose {
            q.print(verbose);
        }
        return a;
    }

    pub fn run(&self) {
        let r: RecordType = RecordType::from_string(self.flags.record.clone());
        let qry = Query::new(
            self.flags.uri.clone(),
            r,
            self.flags.rd.clone(),
        );
        let a = self.send_query(
            qry,
            socket::DNSSocket::from_string(&self.flags.server),
            self.flags.verbose
        );
        let resp = ByteStreamParser::new(&a).parse_response().unwrap();
        resp.print(self.flags.verbose);
    }
}
