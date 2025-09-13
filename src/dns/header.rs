use byters::{BigEndian, ReadsFromBytes};

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
    /*
    A 16 bit identifier assigned by the program that
    generates any kind of query.  This identifier is copied into
    the corresponding reply and can be used by the requester
    to match up replies to outstanding queries.
    */
    pub fn id(&self) -> u16 {
        BigEndian::read_into_u16([self.0[0], self.0[1]])
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
}
