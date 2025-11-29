use crate::{
    client::ClientError,
    dns::{error::DnsError, name::NameError},
};

pub type AskrsResult<T> = Result<T, AskrsError>;

#[derive(Debug)]
pub enum AskrsError {
    RecordError(DnsError),
    DomainNameError(NameError),
    ClientError(ClientError),
}

impl From<ClientError> for AskrsError {
    fn from(value: ClientError) -> Self {
        AskrsError::ClientError(value)
    }
}
