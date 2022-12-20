use utility::{Row,Blob};

use crate::header_flags::Flags;
use crate::header_flags::RCODE;


pub struct Header {
    id: [u8;2],
    flags: Flags,
    q_count: [u8;2],
    an_count: [u8;2],
    ns_count: [u8;2],
    ar_count: [u8;2],
}

impl Header {

    pub fn new_query(rd: Option<bool>) -> Self {
        let mut flags = Flags::new();
        flags.set_to_query();
        if rd.unwrap_or(false) {flags.set_recursive()};
        return Self{
            id: [192, 175],
            flags,
            q_count: [0,1],
            an_count: [0, 0],
            ns_count: [0, 0],
            ar_count: [0, 0],
        };
    }

    pub fn an_count(&self) -> u16 {
        return self.an_count.as_u16();
    }

    pub fn q_count(&self) -> u16 {
        return self.q_count.as_u16();
    }

    pub fn rcode(&self) -> RCODE {
        return self.flags.get_rcode();
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let data: Vec<u8> = bytes.get_slice(0, 12).unwrap();
        return Header{
            id: data[0..2].try_into().unwrap(),
            flags: Flags::from_bytes(data[2..4].try_into().unwrap()),
            q_count: data[4..6].try_into().unwrap(),
            an_count: data[6..8].try_into().unwrap(),
            ns_count: data[8..10].try_into().unwrap(),
            ar_count: data[10..12].try_into().unwrap(),
        };
    }

   pub fn print(&self) {
       println!("ID: {:#01x}", self.id.as_u16());
       println!("Flags: {:#02x}", self.flags.data().as_u16());
       println!("Question count: {}", self.q_count.as_u16());
       println!("Resource Records: {}", self.an_count.as_u16());
       println!("Name Server Records: {}", self.ns_count.as_u16());
       println!("Additional Records: {}", self.ar_count.as_u16());
   } 

   pub fn to_bytes(&self) -> Vec<u8> {
      let mut res: Vec<u8> = vec![];
      res.extend(self.id.to_vec());
      res.extend(self.flags.data().to_vec());
      res.extend(self.q_count.to_vec());
      res.extend(self.an_count.to_vec());
      res.extend(self.ns_count.to_vec());
      res.extend(self.ar_count.to_vec());
      return res;
   }
}
