use std::fmt::Display;

use super::flags::Flags;
use crate::data::Row;

/// Header Section
/*
                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      ID                       |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   | <- Flags
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    QDCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ANCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    NSCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ARCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
*/
pub(crate) struct DnsHeader {
    id: Row,
    flags: Row,
    q_count: Row,
    an_count: Row,
    ns_count: Row,
    ar_count: Row,
}

impl DnsHeader {
    pub(crate) fn id(&self) -> u16 {
        u16::from(self.id)
    }

    pub(crate) fn flags(&self) -> Flags {
        Flags::new(self.flags)
    }

    pub(crate) fn q_count(&self) -> u16 {
        u16::from(self.q_count)
    }

    pub(crate) fn an_count(&self) -> u16 {
        u16::from(self.an_count)
    }

    pub(crate) fn ns_count(&self) -> u16 {
        u16::from(self.ns_count)
    }

    pub(crate) fn ar_count(&self) -> u16 {
        u16::from(self.ar_count)
    }
}

impl Display for DnsHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#02X?}\n{}\n{}\n{}\n{}\n{}",
            self.id(),
            self.flags(),
            self.q_count(),
            self.an_count(),
            self.ns_count(),
            self.an_count(),
        )
    }
}
