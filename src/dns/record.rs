use std::net::{Ipv4Addr, Ipv6Addr};

use crate::dns::name::Name;

pub enum RecordData<'a> {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(Name<'a>),
    MX { preference: u16, exchange: Name<'a> },
    NS(Name<'a>),
    TXT(&'a str),
}

// impl TryFrom<&str> for RecordDataType {
//     type Error = RecordError;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "A" => Ok(RecordDataType::A),
//             "AAAA" => Ok(RecordDataType::AAAA),
//             "CNAME" => Ok(RecordDataType::CNAME),
//             "MX" => Ok(RecordDataType::MX),
//             "NS" => Ok(RecordDataType::NS),
//             _ => Err(RecordError::InvalidRecordDataType),
//         }
//     }
// }

// impl RecordData<'_> {
//     pub fn r#type(&self) -> RecordDataType {
//         match self {
//             RecordData::A(_) => RecordDataType::A,
//             RecordData::AAAA(_) => RecordDataType::AAAA,
//             RecordData::CNAME(_) => RecordDataType::CNAME,
//             RecordData::MX { .. } => RecordDataType::MX,
//             RecordData::NS(_) => RecordDataType::NS,
//         }
//     }
// }

impl From<Ipv4Addr> for RecordData<'_> {
    fn from(value: Ipv4Addr) -> Self {
        Self::A(value)
    }
}

impl From<[u8; 4]> for RecordData<'_> {
    fn from(value: [u8; 4]) -> Self {
        let addr = Ipv4Addr::from_octets(value);
        RecordData::A(addr)
    }
}

impl From<Ipv6Addr> for RecordData<'_> {
    fn from(value: Ipv6Addr) -> Self {
        Self::AAAA(value)
    }
}

impl From<[u8; 16]> for RecordData<'_> {
    fn from(value: [u8; 16]) -> Self {
        let addr = Ipv6Addr::from_octets(value);
        Self::AAAA(addr)
    }
}

impl<'a> From<(u16, Name<'a>)> for RecordData<'a> {
    fn from(value: (u16, Name<'a>)) -> Self {
        Self::MX {
            preference: value.0,
            exchange: value.1,
        }
    }
}

impl<'a> From<&'a str> for RecordData<'a> {
    fn from(value: &'a str) -> Self {
        Self::TXT(value)
    }
}

// impl<'a> TryFrom<(RecordDataType, Name<'a>)> for RecordData<'a> {
//     type Error = RecordError;
//     fn try_from(value: (RecordDataType, Name<'a>)) -> Result<Self, Self::Error> {
//         match value.0 {
//             RecordDataType::CNAME => Ok(RecordData::CNAME(value.1)),
//             RecordDataType::NS => Ok(RecordData::NS(value.1)),
//             _ => Err(RecordError::InvalidRecordDataType),
//         }
//     }
// }
