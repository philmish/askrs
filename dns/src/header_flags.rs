use utility::{Row, Byte};

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

    pub fn is_err(&self) -> bool {
        return match self {
            RCODE::NOERR => false,
            RCODE::FMTERR => true,
            RCODE::SRVFAIL => true,
            RCODE::NAMEERR => true,
            RCODE::NOTIMPL => true,
            RCODE::REFUSED => true,
            RCODE::UNKNOWN => true,
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

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        return Flags{
            bytes: [bytes[0], bytes[1]],
        };
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

    pub fn get_rcode(&self) -> RCODE {
        return RCODE::from_byte(self.bytes[1]);
    }
}
