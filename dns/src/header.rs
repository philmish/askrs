use utility::{Row,Blob};

pub struct Flags {
    bytes: [u8;2],
}

impl Flags {
    
    pub fn new() -> Self {
        return Flags{bytes: [0,0]};
    }

    pub fn set_recursive(&mut self) {
        self.bytes.start_set_bits(0b0000_0001);
    }

    pub fn set_to_query(&mut self) {
        self.bytes.start_set_bits(0b1000_0000);
    }

    pub fn set_standard_query(&mut self) {
        self.bytes.start_set_bits(0b1000_0000);
    }

    pub fn set_standard_inverse(&mut self) {
        self.bytes.start_set_bits(0b1000_1000);
    }
}

pub struct Header {
    id: [u8;2],
    flags: [u8;2],
    q_count: [u8;2],
    an_count: [u8;2],
    ns_count: [u8;2],
    ar_count: [u8;2],
}

impl Header {

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let data: Vec<u8> = bytes.get_slice(0, 12).unwrap();
        return Header{
            id: data[0..2].try_into().unwrap(),
            flags: data[2..4].try_into().unwrap(),
            q_count: data[4..6].try_into().unwrap(),
            an_count: data[6..8].try_into().unwrap(),
            ns_count: data[8..10].try_into().unwrap(),
            ar_count: data[10..12].try_into().unwrap(),
        };
    }

   pub fn print(&self) {
       println!("ID: {:#01}", self.id.as_u16());
       println!("Flags: {:#02}", self.flags.as_u16());
       println!("Question count: {}", self.q_count.as_u16());
       println!("Resource Records: {}", self.an_count.as_u16());
       println!("Name Server Records: {}", self.ns_count.as_u16());
       println!("Additional Records: {}", self.ar_count.as_u16());
   } 
}
