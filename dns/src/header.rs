use utility::{Row,Blob};

pub struct Flags {
    bytes: [u8;2],
}

impl Flags {
    
    pub fn new() -> Self {
        return Flags{bytes: [0,0]};
    }
    
    pub fn data(&self) -> [u8;2] {
        return  self.bytes;
    }

    pub fn set_recursive(&mut self) {
        self.bytes.start_set_bits(0b0000_0001);
    }

    pub fn set_to_query(&mut self) {
        self.bytes.start_set_bits(0b0000_0000);
    }

    pub fn set_standard_query(&mut self) {
        self.bytes.start_set_bits(0b0000_0000);
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

    pub fn new_query(rd: Option<bool>) -> Self {
        let mut flags = Flags::new();
        flags.set_to_query();
        if rd.unwrap_or(false) {flags.set_recursive()};
        return Self{
            id: [192, 175],
            flags: flags.data(),
            q_count: [0,1],
            an_count: [0, 0],
            ns_count: [0, 0],
            ar_count: [0, 0],
        };
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        println!("Parsing headers from {} bytes.", bytes.len());
        let data: Vec<u8> = bytes.get_slice(0, 12).unwrap();
        println!("Header parsing aquired chunk of size {}", data.len());
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

   pub fn to_bytes(&self) -> Vec<u8> {
      let mut res: Vec<u8> = vec![];
      res.extend(self.id.to_vec());
      res.extend(self.flags.to_vec());
      res.extend(self.q_count.to_vec());
      res.extend(self.an_count.to_vec());
      res.extend(self.ns_count.to_vec());
      res.extend(self.ar_count.to_vec());
      return res;
   }
}
