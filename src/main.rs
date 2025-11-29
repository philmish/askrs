pub(crate) mod client;
pub mod dns;
pub mod error;
pub(crate) mod ressources;

use crate::{
    client::{ClientError, DnsClient, default_dns_client},
    dns::{name::Name, question::Question},
    error::AskrsResult,
    ressources::DnsServer,
};

fn main() -> AskrsResult<()> {
    let domain = Name::try_from("microsoft.com")?;
    let q = Question::a_type_record(domain);
    let srv = DnsServer::CLOUDFLARE;
    let mut client = default_dns_client();
    let resp = client.standard_query(q, &srv)?;
    Ok(())
}
