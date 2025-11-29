use std::fmt::Display;

use crate::{
    client::ClientError,
    dns::{error::DnsError, name::NameError},
};

#[derive(Debug)]
pub enum AskrsError {
    RecordError(DnsError),
    DomainNameError(NameError),
    ClientError(ClientError),
}

pub type AskrsResult<T> = Result<T, AskrsError>;

impl std::error::Error for AskrsError {}

impl Display for AskrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ClientError> for AskrsError {
    fn from(value: ClientError) -> Self {
        AskrsError::ClientError(value)
    }
}

impl From<NameError> for AskrsError {
    fn from(value: NameError) -> Self {
        AskrsError::DomainNameError(value)
    }
}
