use std::fmt::Display;

use crate::dns::header::ErrorCode;

#[derive(Debug)]
pub enum DnsError {
    InvalidQR(u8),
    InvalidType(u16),
    InvalidQType(u16),
    InvalidClass(u16),
    InvalidQClass(u16),
    InvalidHeaderLength(usize),
    QueryError(ErrorCode),
}

impl Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DnsError {}
