use utility::Row;

pub mod header;
pub mod name;
pub mod question;
pub mod answer;
pub mod record;
/*
 *
QTYPE           value and meaning

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
AXFR            252 A request for a transfer of an entire zone
MAILB           253 A request for mailbox-related records (MB, MG or MR)
MAILA           254 A request for mail agent RRs (Obsolete - see MX)
*               255 A request for all records

 *
 */
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
}

impl RecordType {

    pub fn as_bytes(r_type: RecordType) -> [u8;2] {
        match r_type {
            RecordType::A => [0b0000_0000, 0b0000_0001],
            RecordType::AAAA => [0b0000_0000, 0b0001_1100],
            RecordType::CNAME => [0b0000_0000, 0b0000_0101],
            RecordType::NS => [0b0000_0000, 0b0000_0010],
            RecordType::MX => [0b0000_0000, 0b0000_1111],
        }
    }

    pub fn from_string(r_type: String) -> Self {
        match r_type.as_str() {
            "A" => RecordType::A,
            "AAAA" => RecordType::AAAA,
            "CNAME" => RecordType::CNAME,
            "MX" => RecordType::MX,
            "NS" => RecordType::NS,
            _ => RecordType::A
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RecordType::A => "A".to_string(),
            RecordType::AAAA => "AAAA".to_string(),
            RecordType::CNAME => "CNAME".to_string(),
            RecordType::MX => "MX".to_string(),
            RecordType::NS => "NS".to_string(),
        }
    }

    pub fn from_bytes(data: [u8;2]) -> Self {
        match data.as_u16() {
            1 => RecordType::A,
            2 => RecordType::NS,
            28 => RecordType::AAAA,
            5 => RecordType::CNAME,
            15 => RecordType::MX,
            _ => RecordType::A,
        }
    }
}
