use utility::{Row,Blob, Byte};

pub enum RCODE {
    NOERR,
    FMTERR,
    SRVFAIL,
    NAMEERR,
    NOTIMPL,
    REFUSED,
    UNKNOWN,
}

impl RCODE {
    
    pub fn from_byte(byte: u8) -> RCODE {
        let n: u8 = byte.take_right_nibble();
        return match n {
            0 => RCODE::NOERR,
            1 => RCODE::FMTERR,
            2 => RCODE::SRVFAIL,
            3 => RCODE::NAMEERR,
            4 => RCODE::NOTIMPL,
            5 => RCODE::REFUSED,
            _ => RCODE::UNKNOWN,
        };
    }

    pub fn print(&self) {
        match self {
            RCODE::NOERR => println!("No Errors encountered."),
            RCODE::FMTERR => println!("The Name Server was unable to interpret the query."),
            RCODE::SRVFAIL => println!("The Name Server was unable to process this query due to a problem with the name server."),
            RCODE::NAMEERR => println!("Meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist."),
            RCODE::NOTIMPL => println!("The name server does not support the requested kind of query."),
            RCODE::REFUSED => println!("The name server refuses to perform the specified operation for policy reasons.  For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (e.g., zone"),
            RCODE::UNKNOWN => println!("An unknown error code was encountered. Check your parsing.")
        }
    }
}

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

    pub fn an_count(&self) -> u16 {
        return self.an_count.as_u16();
    }

    pub fn q_count(&self) -> u16 {
        return self.q_count.as_u16();
    }

    pub fn rcode(&self) -> RCODE {
        return RCODE::from_byte(self.flags[1]);
    }

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
       println!("ID: {:#01x}", self.id.as_u16());
       println!("Flags: {:#02x}", self.flags.as_u16());
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
