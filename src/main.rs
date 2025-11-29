use crate::{client::ClientError, error::AskrsResult};

pub(crate) mod client;
pub mod dns;
pub mod error;
pub(crate) mod ressources;

fn main() -> AskrsResult<()> {
    println!("hello world");
    Err(ClientError::NoResponse.into())
}
