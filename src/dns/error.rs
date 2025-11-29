use std::fmt::Display;

use crate::dns::{header::ErrorCode, name::NameError};

#[derive(Debug)]
pub enum DnsError {
    InvalidQR(u8),
    InvalidType(u16),
    InvalidQType(u16),
    InvalidClass(u16),
    InvalidQClass(u16),
    InvalidHeaderLength(usize),
    QueryError(ErrorCode),
    InvalidDomainName(NameError),
}

impl std::error::Error for DnsError {}

impl Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<NameError> for DnsError {
    fn from(value: NameError) -> Self {
        DnsError::InvalidDomainName(value)
    }
}
