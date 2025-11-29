use byters::{
    BigEndian, BitOffset, BitRange, BitsReadableAs, ReadableBit, ReadsFromBytes, ReadsIntoBytes,
    SetableBit,
};

use crate::dns::error::DnsError;

/**
DNS Message Header. Included in queries, as well as responses.

Uses the following format:
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
pub struct MessageHeader([u8; 12]);

impl MessageHeader {
    pub fn empty() -> Self {
        return Self([0u8; 12]);
    }
}

impl From<[u8; 12]> for MessageHeader {
    fn from(value: [u8; 12]) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for MessageHeader {
    type Error = DnsError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Some(v) = value.chunks_exact(12).next() {
            let b: [u8; 12] = v
                .try_into()
                .map_err(|_| DnsError::InvalidHeaderLength(v.len()))?;
            return Ok(Self(b));
        }
        Err(DnsError::InvalidHeaderLength(value.len()))
    }
}

/// Helper struct for reading and writing fields to and from a
/// `MessageHeader`
pub struct HeaderReadWriter {
    bits: HeaderBits,
}

impl HeaderReadWriter {
    pub fn new() -> Self {
        Self {
            bits: HeaderBits::new(),
        }
    }

    /*
    Read the 16bit identifier from the record header data
    */
    pub fn read_id(&self, data: &MessageHeader) -> u16 {
        BigEndian::read_into_u16([data.0[0], data.0[1]])
    }

    /*
    Writes the provided 16 bit identifier for the query into the given
    header data
    */
    pub fn write_id(&self, id: u16, data: &mut MessageHeader) {
        data.0[0..2].copy_from_slice(BigEndian::read_from_u16(id).as_slice());
    }

    /*
    Read the 16 bit integer specifying the number of
    entries in the question section.
    */
    pub fn read_qd_count(&self, data: &MessageHeader) -> u16 {
        BigEndian::read_into_u16([data.0[4], data.0[5]])
    }

    /*
    Write the amount of entries in the question section into the header
    data
    */
    pub fn write_qd_count(&self, n: u16, data: &mut MessageHeader) {
        data.0[4..=5].copy_from_slice(BigEndian::read_from_u16(n).as_slice());
    }

    /*
    Read the 16 bit integer specifying the number of
    resource records in the answer section.
    */
    pub fn read_an_count(&self, data: &MessageHeader) -> u16 {
        BigEndian::read_into_u16([data.0[6], data.0[7]])
    }

    /*
    Read the 16 bit integer specifying the number of name
    server resource records in the authority records
    section.
    */
    pub fn read_ns_count(&self, data: &MessageHeader) -> u16 {
        BigEndian::read_into_u16([data.0[8], data.0[9]])
    }

    /*
    Read the 16 bit integer specifying the number of
    resource records in the additional records section.
    */
    pub fn ar_count(&self, data: &MessageHeader) -> u16 {
        BigEndian::read_into_u16([data.0[10], data.0[11]])
    }

    /*
    Try to read the bits specifying the type of Query. Returns an error
    if an invalid value is read from the header.
    */
    pub fn try_read_qr(&self, data: &MessageHeader) -> Result<Qr, DnsError> {
        match data.0[2].read_bit(self.bits.qr_offset) {
            0 => Ok(Qr::Query),
            1 => Ok(Qr::Response),
            b => Err(DnsError::InvalidQR(b)),
        }
    }

    pub fn read_opcode(&self, data: &MessageHeader) -> Opcode {
        let op: u8 = data.0[2].read_bits_as(self.bits.op_range);
        Opcode::from(op)
    }
    pub fn write_opcode(&self, op: Opcode, data: &mut MessageHeader) {
        let mask: u8 = op.into();
        data.0[2] |= mask << 3;
    }

    pub fn is_authoritive(&self, data: &MessageHeader) -> bool {
        data.0[2].read_bit(self.bits.auth_offset) == 1
    }

    pub fn is_truncated(&self, data: &MessageHeader) -> bool {
        data.0[2].read_bit(self.bits.trunc_offset) == 1
    }

    pub fn recursion_is_desired(&self, data: &MessageHeader) -> bool {
        data.0[2].read_bit(self.bits.recd_offset) == 1
    }

    pub fn write_recursion_desired(&self, data: &mut MessageHeader) {
        data.0[2].set_bit(self.bits.rec_offset);
    }

    pub fn recursion_is_available(&self, data: &MessageHeader) -> bool {
        data.0[3].read_bit(self.bits.rec_offset) == 1
    }

    pub fn read_error(&self, data: &MessageHeader) -> Option<ErrorCode> {
        match BitsReadableAs::<u8>::read_bits_as(&data.0[3], self.bits.err_range) {
            1 => Some(ErrorCode::FormatError),
            2 => Some(ErrorCode::ServerFailure),
            3 => Some(ErrorCode::NameError),
            4 => Some(ErrorCode::NotImplemented),
            5 => Some(ErrorCode::Refused),
            _ => None,
        }
    }
}

/// Specifier if a Message is either a `Query` or a `Response` to
/// a `Query`
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

#[derive(Debug)]
pub enum ErrorCode {
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}

impl From<ErrorCode> for DnsError {
    fn from(value: ErrorCode) -> Self {
        DnsError::QueryError(value)
    }
}

struct HeaderBits {
    err_range: BitRange<u8>,
    op_range: BitRange<u8>,
    rec_offset: BitOffset<u8>,
    qr_offset: BitOffset<u8>,
    auth_offset: BitOffset<u8>,
    trunc_offset: BitOffset<u8>,
    recd_offset: BitOffset<u8>,
}

impl HeaderBits {
    pub fn new() -> Self {
        // SAFTEY: we can unwrap as we know all ranges and offsets are valid
        Self {
            err_range: BitRange::<u8>::new(0, 3).unwrap(),
            op_range: BitRange::<u8>::new(3, 6).unwrap(),
            rec_offset: BitOffset::<u8>::new(7).unwrap(),
            qr_offset: BitOffset::<u8>::new(7).unwrap(),
            auth_offset: BitOffset::<u8>::new(2).unwrap(),
            trunc_offset: BitOffset::<u8>::new(1).unwrap(),
            recd_offset: BitOffset::<u8>::new(0).unwrap(),
        }
    }
}
