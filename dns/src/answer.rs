use utility::{Row, Blob};

use crate::{name::Name, record::RecordType};


trait ParsableRecord {
    fn read_r_data(data: Vec<u8>, offset: Option<u8>) -> Self;
    fn print_r_data(&self);
}

struct RecordData<T: ParsableRecord> {
    content: T,
}

impl ParsableRecord for Name {
    fn print_r_data(&self) {
        println!("Name: {}", self.get_string().unwrap_or("is unparseable".to_string()));
    }

    fn read_r_data(data: Vec<u8>, offset: Option<u8>) -> Self {
        let bytes = data.get_from_offset(offset.unwrap_or(0)).unwrap();
        return Name::from_bytes(bytes);
    }
}

//TODO Find a better way to parse answer data for Address or CNAME
pub struct Answer {
    name: Name,
    r_type: RecordType,
    class: [u8;2],
    ttl: [u8;4],
    length: [u8;2],
    _data: Vec<u8>,
}

impl Answer {

    fn ttl_as_u32(&self) -> u32 {
        ((self.ttl[0] as u32) << 24) +
        ((self.ttl[1] as u32) << 16) +
        ((self.ttl[2] as u32) << 8) +
        ((self.ttl[3] as u32) << 0)
    }
    
    pub fn print(&self) {
        println!("Name: {}", self.name.get_string().unwrap());
        println!("Type: {}", self.r_type.to_string());
        println!("Class: {}", self.class.as_u16());
        println!("TTL: {}", self.ttl_as_u32());
        println!("Length: {}", self.length.as_u16());
    }
}
