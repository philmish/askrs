use utility::Row;

use crate::{
    name::Name,
    record::{AAAARecord, ARecord, CNAMERecord, MXRecord, NSRecord, RecordType},
};

/*
                               1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                                               |
    /                                               /
    /                      NAME                     /
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     CLASS                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TTL                      |
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                   RDLENGTH                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
    /                     RDATA                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

*/

//TODO make length a u16 and ttl a u32 as they are used as such
pub struct Answer {
    name: Name,
    r_type: RecordType,
    class: [u8; 2],
    ttl: [u8; 4],
    length: [u8; 2],
    a_data: Vec<u8>,
}

impl Clone for Answer {
    fn clone(&self) -> Self {
        return Answer {
            name: self.name.clone(),
            r_type: self.r_type.clone(),
            class: self.class.clone(),
            ttl: self.ttl.clone(),
            length: self.length.clone(),
            a_data: self.a_data.to_vec(),
        };
    }
}

impl Answer {
    pub fn new(
        name: Name,
        r_type: RecordType,
        class: [u8; 2],
        ttl: [u8; 4],
        length: [u8; 2],
        a_data: Vec<u8>,
    ) -> Self {
        return Self {
            name,
            r_type,
            class,
            ttl,
            length,
            a_data,
        };
    }

    fn ttl_as_u32(&self) -> u32 {
        ((self.ttl[0] as u32) << 24)
            + ((self.ttl[1] as u32) << 16)
            + ((self.ttl[2] as u32) << 8)
            + ((self.ttl[3] as u32) << 0)
    }

    fn print_record(&self, src: Vec<u8>) {
        match self.r_type.to_string().as_str() {
            "A" => ARecord::from_bytes(self.a_data.to_vec(), 0).print(),
            "AAAA" => AAAARecord::from_bytes(self.a_data.to_vec(), 0).print(),
            "CNAME" => CNAMERecord::from_bytes(self.a_data.to_vec(), src, 0).print(),
            "MX" => MXRecord::from_bytes(self.a_data.to_vec(), src, 0).print(),
            "NS" => NSRecord::from_bytes(self.a_data.to_vec(), src, 0).print(),
            _ => println!("\tunparseable answer data."),
        };
    }

    pub fn print(&self, src: Vec<u8>) {
        println!("---------------------");
        println!("\tName: {}", self.name.get_string().unwrap());
        println!("\tType: {}", self.r_type.to_string());
        println!("\tClass: {}", self.class.as_u16());
        println!("\tTTL: {}", self.ttl_as_u32());
        println!("\tLength: {}", self.length.as_u16());
        self.print_record(src);
        println!("---------------------");
    }
}
