use crate::dns::error::DnsError;

/*
TYPE            value and meaning
A               1 a host address
NS              2 an authoritative name server
MD              3 a mail destination (Obsolete - use MX)
MF              4 a mail forwarder (Obsolete - use MX)
CNAME           5 the canonical name for an alias
SOA             6 marks the start of a zone of authority
MB              7 a mailbox domain name (EXPERIMENTAL)
MG              8 a mail group member (EXPERIMENTAL)
MR              9 a mail rename domain name (EXPERIMENTAL)
NULL            10 a null RR (EXPERIMENTAL)
WKS             11 a well known service description
PTR             12 a domain name pointer
HINFO           13 host information
MINFO           14 mailbox or mail list information
MX              15 mail exchange
TXT             16 text strings
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordType {
    A,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    AAAA,
    AFXR,
    MAILB,
    MAILA,
    ANY,
}

/// new type used to pass around a record type as a valid
/// `Type`.
#[derive(Debug)]
pub struct Type(RecordType);

impl Into<u16> for Type {
    fn into(self) -> u16 {
        self.0.into()
    }
}

impl TryFrom<RecordType> for Type {
    type Error = DnsError;

    fn try_from(value: RecordType) -> Result<Self, Self::Error> {
        match value {
            RecordType::AFXR | RecordType::MAILB | RecordType::MAILA | RecordType::ANY => {
                Err(DnsError::InvalidType(value as u16))
            }
            _ => Ok(Self(value)),
        }
    }
}

impl TryFrom<u16> for Type {
    type Error = DnsError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let rt = RecordType::try_from(value).map_err(|_| DnsError::InvalidType(value))?;
        Self::try_from(rt)
    }
}

/// new type used to pass around a record type as a valid
/// `QType`
#[derive(Clone, Copy, Debug)]
pub struct QType(RecordType);

impl Into<u16> for QType {
    fn into(self) -> u16 {
        self.0.into()
    }
}

impl From<RecordType> for QType {
    fn from(value: RecordType) -> Self {
        Self(value)
    }
}

impl TryFrom<u16> for QType {
    type Error = DnsError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match RecordType::try_from(value) {
            Ok(t) => Ok(Self(t)),
            Err(_) => Err(DnsError::InvalidType(value)),
        }
    }
}

impl TryFrom<u16> for RecordType {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::NS),
            3 => Ok(Self::MD),
            4 => Ok(Self::MF),
            5 => Ok(Self::CNAME),
            6 => Ok(Self::SOA),
            7 => Ok(Self::MB),
            8 => Ok(Self::MG),
            9 => Ok(Self::MR),
            10 => Ok(Self::NULL),
            11 => Ok(Self::WKS),
            12 => Ok(Self::PTR),
            13 => Ok(Self::HINFO),
            14 => Ok(Self::MINFO),
            15 => Ok(Self::MX),
            15 => Ok(Self::TXT),
            28 => Ok(Self::AAAA),
            252 => Ok(Self::AFXR),
            253 => Ok(Self::MAILB),
            254 => Ok(Self::MAILA),
            255 => Ok(Self::ANY),
            _ => Err(()),
        }
    }
}

impl Into<u16> for RecordType {
    fn into(self) -> u16 {
        match self {
            Self::A => 1,
            Self::NS => 2,
            Self::MD => 3,
            Self::MF => 4,
            Self::CNAME => 5,
            Self::SOA => 6,
            Self::MB => 7,
            Self::MG => 8,
            Self::MR => 9,
            Self::NULL => 10,
            Self::WKS => 11,
            Self::PTR => 12,
            Self::HINFO => 13,
            Self::MINFO => 14,
            Self::MX => 15,
            Self::TXT => 16,
            Self::AAAA => 28,
            Self::AFXR => 252,
            Self::MAILB => 253,
            Self::MAILA => 254,
            Self::ANY => 255,
        }
    }
}
