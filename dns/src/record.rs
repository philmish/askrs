use std::net::{Ipv4Addr, Ipv6Addr};
use utility::{Row, Blob};

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
//TODO implement CNAME, MX and NS Records

pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
}

impl Clone for RecordType {
    fn clone(&self) -> Self {
        match self {
            RecordType::A => RecordType::A,
            RecordType::AAAA => RecordType::AAAA,
            RecordType::CNAME => RecordType::CNAME,
            RecordType::NS => RecordType::NS,
            RecordType::MX => RecordType::MX,
        }
    }
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

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            RecordType::A => vec![0,1],
            RecordType::NS => vec![0b0000_0000, 0b0000_0010],
            RecordType::CNAME => vec![0b0000_0000, 0b0000_0101],
            RecordType::AAAA => vec![0b0000_0000, 0b0001_1100],
            RecordType::MX => vec![0b0000_0000, 0b0000_1111],
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

pub struct ARecord {
    fields: [u8;4]
}

impl ARecord {
    
    pub fn from_bytes(data: Vec<u8>, offset: u8) -> Self {
        let bytes: Vec<u8> = data.to_vec().get_from_offset(offset).unwrap();
        let mut pos = 0;
        let mut fields: [u8;4];
        let mut i = bytes.into_iter();
        if i.len() < 4  {
            panic!("Parsing A Record from iterator with len {}", i.len());
        }
        fields = [0,0,0,0];
        while pos < 4 {
            fields[pos] = i.next().unwrap();
            pos += 1;
        }
        return ARecord{fields};
    }

    pub fn as_ipv4(&self) -> Ipv4Addr {
        return Ipv4Addr::new(self.fields[0], self.fields[1], self.fields[2], self.fields[3]);
    }

    pub fn print(&self) {
        println!("IPv4: {}", self.as_ipv4());
    }
}

pub struct AAAARecord {
    bytes: [u8; 16]
}

impl AAAARecord {

    pub fn from_bytes(data: Vec<u8>, offset: u8) -> Self {
        let bytes = data.to_vec().get_from_offset(offset).unwrap();
        let mut pos: usize = 0;
        let mut buf: [u8;16];
        let mut iter = bytes.into_iter();
        if iter.len() < 16 {
            panic!("Trying to parse AAAA record from byte vector of length {}", iter.len());
        }
        buf = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        while pos < 16 {
            buf[pos] = iter.next().unwrap();
            pos += 1;
        }
        return Self { bytes: buf };
    }

    pub fn as_ipv6(&self) -> Ipv6Addr {
        let mut fields: [u16;8] = [0,0,0,0,0,0,0,0];
        let mut pos: usize = 0;
        let mut buf: [u8;2];
        for el in self.bytes.chunks(2) {
            buf = [el[0], el[1]];
            fields[pos] = buf.as_u16();
            pos += 1;
        }
        return Ipv6Addr::new(
            fields[0],
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
            fields[6],
            fields[7]
        );
    }

    pub fn print(&self) {
        println!("IPv6: {}", self.as_ipv6());
    }
}

#[cfg(test)]
mod tests {

    use super::AAAARecord;

    #[test]
    fn test_aaaa_record() {
        let data: Vec<u8> = vec![244, 144, 131, 10, 253, 198, 107, 97, 126, 155, 106, 122, 200, 157, 89, 237];
        let record = AAAARecord::from_bytes(data.to_vec(), 0);
        record.print();
    }
}
