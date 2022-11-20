use utility::Row;

use crate::name::Name;

//TODO Find a better way to parse answer data for Address or CNAME
pub struct Answer {
    name: Name,
    r_type: [u8;2],
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
        println!("Type: {}", self.r_type.as_u16());
        println!("Class: {}", self.class.as_u16());
        println!("TTL: {}", self.ttl_as_u32());
        println!("Length: {}", self.length.as_u16());
    }
}
