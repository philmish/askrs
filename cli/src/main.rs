use clap::Parser;
use cli::CLI;

#[derive(Parser, Debug)]
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

fn main() {
    let client = CLI::init();
    client.run();
}
