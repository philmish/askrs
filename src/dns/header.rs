use byters::{BigEndian, BitsReadableAs, ReadableBit, ReadsFromBytes, ReadsIntoBytes, SetableBit};

/**
                                 1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      ID                       |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    QDCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ANCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    NSCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ARCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

**/
pub struct Header([u8; 12]);

impl Header {
    pub fn new() -> Self {
        Header([0u8; 12])
    }
    /*
    A 16 bit identifier assigned by the program that
    generates any kind of query.  This identifier is copied into
    the corresponding reply and can be used by the requester
    to match up replies to outstanding queries.
    */
    pub fn id(&self) -> u16 {
        BigEndian::read_into_u16([self.0[0], self.0[1]])
    }

    pub fn set_id(&mut self, id: u16) {
        let bytes = BigEndian::read_from_u16(id);
        self.0[0..2].copy_from_slice(bytes.as_slice());
    }

    /*
    an unsigned 16 bit integer specifying the number of
    entries in the question section.
    */
    pub fn qd_count(&self) -> u16 {
        BigEndian::read_into_u16([self.0[4], self.0[5]])
    }

    /*
    an unsigned 16 bit integer specifying the number of
    resource records in the answer section.
    */
    pub fn an_count(&self) -> u16 {
        BigEndian::read_into_u16([self.0[6], self.0[7]])
    }

    /*
    an unsigned 16 bit integer specifying the number of name
    server resource records in the authority records
    section.
    */
    pub fn ns_count(&self) -> u16 {
        BigEndian::read_into_u16([self.0[8], self.0[9]])
    }

    /*
    an unsigned 16 bit integer specifying the number of
    resource records in the additional records section.
    */
    pub fn ar_count(&self) -> u16 {
        BigEndian::read_into_u16([self.0[10], self.0[11]])
    }

    /// copy header data as bytes into buffer.
    /// returns the amount of bytes copied, which is
    /// 12 in all cases.
    pub fn copy_bytes(&self, buf: &mut [u8]) -> usize {
        buf.copy_from_slice(&self.0[..]);
        12
    }

    pub fn qr(&self) -> Qr {
        match self.0[2].read_bit(7) {
            0 => Qr::Query,
            1 => Qr::Response,
            b => panic!("invalid qr bit {b}"),
        }
    }

    pub fn opcode(&self) -> Opcode {
        let op: u8 = self.0[2].read_bits_as(3, 6);
        Opcode::from(op)
    }

    pub fn set_opcode(&mut self, op: Opcode) {
        let mask: u8 = op.into();
        self.0[2] |= mask << 3;
    }

    pub fn is_authoritive(&self) -> bool {
        self.0[2].read_bit(2) == 1
    }

    pub fn is_truncated(&self) -> bool {
        self.0[2].read_bit(1) == 1
    }

    pub fn recursion_is_desired(&self) -> bool {
        self.0[2].read_bit(0) == 1
    }

    pub fn set_recursion_desired(&mut self) {
        self.0[2].set_bit(0);
    }

    pub fn recursion_is_available(&self) -> bool {
        self.0[3].read_bit(7) == 1
    }

    pub fn error(&self) -> Option<ErrorCode> {
        match BitsReadableAs::<u8>::read_bits_as(&self.0[3], 0, 3) {
            1 => Some(ErrorCode::FormatError),
            2 => Some(ErrorCode::ServerFailure),
            3 => Some(ErrorCode::NameError),
            4 => Some(ErrorCode::NotImplemented),
            5 => Some(ErrorCode::Refused),
            _ => None,
        }
    }
}

pub enum Qr {
    Query,
    Response,
}

pub enum Opcode {
    StandardQuery,
    InverseQuery,
    Status,
    Reserved(u8),
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::StandardQuery,
            1 => Opcode::InverseQuery,
            2 => Opcode::Status,
            b => Opcode::Reserved(b),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Self::StandardQuery => 0,
            Self::InverseQuery => 1,
            Self::Status => 2,
            Self::Reserved(b) => b,
        }
    }
}

pub enum ErrorCode {
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}
