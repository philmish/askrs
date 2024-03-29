use std::net::{Ipv4Addr, Ipv6Addr};
use utility::{Row, Blob};
use crate::name::Name;

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
        let bytes: Vec<u8> = data.to_vec()
                                 .get_from_offset(offset)
                                 .unwrap();
        let mut pos = 0;
        let mut fields: [u8;4] = [0;4];
        let mut i = bytes.into_iter();
        if i.len() < 4  {
            panic!("Parsing A Record from iterator with len {}", i.len());
        }
        while pos < 4 {
            fields[pos] = i.next().unwrap();
            pos += 1;
        }
        return ARecord{fields};
    }

    pub fn as_ipv4(&self) -> Ipv4Addr {
        return Ipv4Addr::new(
            self.fields[0],
            self.fields[1],
            self.fields[2],
            self.fields[3]
        );
    }

    pub fn print(&self) {
        println!("\tIPv4: {}", self.as_ipv4());
    }
}

pub struct AAAARecord {
    bytes: [u8; 16]
}

impl AAAARecord {

    pub fn from_bytes(data: Vec<u8>, offset: u8) -> Self {
        let bytes = data.to_vec().get_from_offset(offset).unwrap();
        let mut pos: usize = 0;
        let mut buf: [u8;16] = [0;16];
        let mut iter = bytes.into_iter();
        if iter.len() < 16 {
            panic!("Trying to parse AAAA record from byte vector of length {}", iter.len());
        }
        while pos < 16 {
            buf[pos] = iter.next().unwrap();
            pos += 1;
        }
        return Self { bytes: buf };
    }

    pub fn as_ipv6(&self) -> Ipv6Addr {
        let mut fields: [u16;8] = [0;8];
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
        println!("\tIPv6: {}", self.as_ipv6());
    }
}

pub struct CNAMERecord {
    name: Name,
}


impl CNAMERecord {

    pub fn from_bytes(data: Vec<u8>, src: Vec<u8>, offset: u8) -> Self {
        let mut name = Name::from_bytes(data.to_vec(), offset);
        if name.is_compressed() {
            name = name.decompress(src.to_vec()).unwrap();
        }
        return Self{name};
    }

    pub fn print(&self) {
       println!("\tName: {}", self.name.get_string().unwrap());
    }
}

pub struct MXRecord {
    preference: u16,
    exchange: Name,
}

impl MXRecord {
    
    pub fn from_bytes(data: Vec<u8>, src: Vec<u8>, offset: u8) -> Self {
        let pref_bytes: Vec<u8> = data.to_vec()
                                  .get_from_offset(offset)
                                  .unwrap()
                                  .get_slice(0, 2)
                                  .unwrap();
        let mut name = Name::from_bytes(data.to_vec(), offset + 2);
        if name.is_compressed() {
            name = name.decompress(src).unwrap();
        }
        let pref: u16 = [pref_bytes[0], pref_bytes[1]].as_u16();
        return Self { preference: pref , exchange: name };
    }

    pub fn print(&self) {
        println!("\tPreference: {}", self.preference);
        println!("\tExchange: {}", self.exchange.get_string().unwrap());
    }
}

pub struct NSRecord {
    nsdname: Name,
}

impl NSRecord {
    pub fn from_bytes(data: Vec<u8>, src: Vec<u8>, offset: u8) -> Self {
        let mut nsdname = Name::from_bytes(data.to_vec(), offset);
        if nsdname.is_compressed() {
            nsdname = nsdname.decompress(src.to_vec()).unwrap();
        }
        return Self{nsdname};
    }

    pub fn print(&self) {
        println!("\tNameserver: {}", self.nsdname.get_string().unwrap());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_aaaa_record() {
        let data: Vec<u8> = vec![
            244, 144, 131, 10,
            253, 198, 107, 97,
            126, 155, 106, 122,
            200, 157, 89, 237
        ];
        let record = AAAARecord::from_bytes(data.to_vec(), 0);
        record.print();
    }

    #[test]
    #[should_panic]
    fn test_aaaa_to_few_bytes() {
        let data: Vec<u8> = vec![
            244, 144, 131, 10,
            253, 198, 107, 97,
            126, 155, 106, 122,
        ];
        let _record = AAAARecord::from_bytes(data.to_vec(), 0);
        
    }

    #[test]
    fn test_record_type_from_bytes() {

        assert!(matches!(RecordType::from_bytes([0,1]), RecordType::A));
        assert!(matches!(RecordType::from_bytes([0,2]), RecordType::NS));
        assert!(matches!(RecordType::from_bytes([0,5]), RecordType::CNAME));
        assert!(matches!(RecordType::from_bytes([0,28]), RecordType::AAAA));
        assert!(matches!(RecordType::from_bytes([0,15]), RecordType::MX));
        assert!(matches!(RecordType::from_bytes([0,245]), RecordType::A));
        
    }

    #[test]
    fn test_record_type_to_string() {
        
            assert_eq!(RecordType::A.to_string(), "A".to_string());
            assert_eq!(RecordType::AAAA.to_string(), "AAAA".to_string());
            assert_eq!(RecordType::CNAME.to_string(), "CNAME".to_string());
            assert_eq!(RecordType::MX.to_string(), "MX".to_string());
            assert_eq!(RecordType::NS.to_string(), "NS".to_string());

    }

    #[test]
    fn test_record_type_from_string() {

       assert!(matches!(RecordType::from_string("A".to_string()), RecordType::A));
       assert!(matches!(RecordType::from_string("AAAA".to_string()), RecordType::AAAA));
       assert!(matches!(RecordType::from_string("CNAME".to_string()), RecordType::CNAME));
       assert!(matches!(RecordType::from_string("MX".to_string()), RecordType::MX));
       assert!(matches!(RecordType::from_string("NS".to_string()), RecordType::NS));

    }
}
