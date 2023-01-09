use utility::{Row, Byte};

/*
 *                                1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

QR              A one bit field that specifies whether this message is a
                query (0), or a response (1).

OPCODE          A four bit field that specifies kind of query in this
                message.  This value is set by the originator of a query
                and copied into the response.  The values are:

                0               a standard query (QUERY)

                1               an inverse query (IQUERY)

                2               a server status request (STATUS)

                3-15            reserved for future use

AA              Authoritative Answer - this bit is valid in responses,
                and specifies that the responding name server is an
                authority for the domain name in question section.

                Note that the contents of the answer section may have
                multiple owner names because of aliases.  The AA bit
                corresponds to the name which matches the query name, or
                the first owner name in the answer section.

TC              TrunCation - specifies that this message was truncated
                due to length greater than that permitted on the
                transmission channel.

RD              Recursion Desired - this bit may be set in a query and
                is copied into the response.  If RD is set, it directs
                the name server to pursue the query recursively.
                Recursive query support is optional.

RA              Recursion Available - this be is set or cleared in a
                response, and denotes whether recursive query support is
                available in the name server.

Z               Reserved for future use.  Must be zero in all queries
                and responses.

RCODE           Response code - this 4 bit field is set as part of
                responses.  The values have the following
                interpretation:

                0               No error condition

                1               Format error - The name server was
                                unable to interpret the query.

                2               Server failure - The name server was
                                unable to process this query due to a
                                problem with the name server.

                3               Name Error - Meaningful only for
                                responses from an authoritative name
                                server, this code signifies that the
                                domain name referenced in the query does
                                not exist.

                4               Not Implemented - The name server does
                                not support the requested kind of query.

                5               Refused - The name server refuses to
                                perform the specified operation for
                                policy reasons.  For example, a name
                                server may not wish to provide the
                                information to the particular requester,
                                or a name server may not wish to perform
                                a particular operation (e.g., zonetransfer) for particular data.

                6-15            Reserved for future use.
 *
 *
 */

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
