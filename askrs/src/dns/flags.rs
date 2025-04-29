use crate::{bits::HasBits, data::Row};
use std::fmt;

/*

                                    1  1  1  1  1  1
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
/// DNS Header Flags
pub(crate) struct Flags(Row);

impl From<(u8, u8)> for Flags {
    fn from(value: (u8, u8)) -> Self {
        Self::new(Row::new(value.0, value.1))
    }
}

impl Flags {
    pub(crate) fn new(row: Row) -> Self {
        Self(row)
    }

    pub(crate) fn is_query(&self) -> bool {
        !self.0.head().bit_is_set(7)
    }

    pub(crate) fn is_response(&self) -> bool {
        self.0.head().bit_is_set(7)
    }

    pub(crate) fn opcode(&self) -> Opcode {
        Opcode::from(self.0.head())
    }

    pub(crate) fn is_authoritative_answer(&self) -> bool {
        self.0.head().bit_is_set(2)
    }

    pub(crate) fn is_truncated(&self) -> bool {
        self.0.head().bit_is_set(1)
    }

    pub(crate) fn recursion_desired(&self) -> bool {
        (self.0.head() & 1) == 1
    }

    pub(crate) fn recursion_available(&self) -> bool {
        self.0.tail().bit_is_set(7)
    }

    pub(crate) fn rcode(&self) -> RCODE {
        RCODE::from(self.0.tail())
    }

    pub(crate) fn has_error(&self) -> bool {
        self.rcode() != RCODE::NONE
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Opcode {
    QUERY,
    IQUERY,
    STATUS,
    RESERVED,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match (value >> 3) & 0x0f {
            0 => Self::QUERY,
            1 => Self::IQUERY,
            2 => Self::STATUS,
            _ => Self::RESERVED,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QUERY => write!(f, "Query"),
            Self::IQUERY => write!(f, "Inverse Query"),
            Self::STATUS => write!(f, "Server Status Request"),
            Self::RESERVED => write!(f, "Reserved"),
        }
    }
}

/// Error Codes returned in response headers
#[derive(PartialEq, Eq, Debug)]
pub(crate) enum RCODE {
    NONE,
    FORMAT,
    SERVER,
    NAME,
    NOIMPL,
    REFUSED,
    RESERVED,
}

impl From<u8> for RCODE {
    fn from(value: u8) -> Self {
        match value & 0x0f {
            0 => Self::NONE,
            1 => Self::FORMAT,
            2 => Self::SERVER,
            3 => Self::NAME,
            4 => Self::NOIMPL,
            5 => Self::REFUSED,
            _ => Self::RESERVED,
        }
    }
}

impl fmt::Display for RCODE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RCODE::NONE => write!(f, "No Errors encountered."),
            RCODE::FORMAT => write!(f, "The Name Server was unable to interpret the query."),
            RCODE::SERVER => write!(
                f,
                "The Name Server was unable to process this query due to a problem with the name server."
            ),
            RCODE::NAME => write!(
                f,
                "Meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist."
            ),
            RCODE::NOIMPL => write!(
                f,
                "The name server does not support the requested kind of query."
            ),
            RCODE::REFUSED => write!(
                f,
                "The name server refuses to perform the specified operation for policy reasons.  For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (e.g., zone"
            ),
            RCODE::RESERVED => write!(
                f,
                "An unknown error code was encountered. Check your parsing."
            ),
        }
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = format!("Opcode: {}\n", self.opcode());
        if self.is_authoritative_answer() {
            out += "Authoritative: True\n";
        } else {
            out += "Authoritative: False\n";
        }

        if self.recursion_desired() {
            out += "Recursion Desired: True\n";
        } else {
            out += "Recursion Desired: False\n";
        }

        if self.recursion_available() {
            out += "Recursion Available: True\n";
        } else {
            out += "Recursion Available: False\n";
        }

        return write!(f, "{}\n{}", out, self.rcode());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_check_qr() {
        let r = Flags::from((0b1000_0000, 0));
        let q = Flags::from((0, 0));

        assert!(r.is_response());
        assert!(!r.is_query());
        assert!(q.is_query());
        assert!(!q.is_response());
    }

    #[test]
    fn can_check_is_truncated() {
        let t = Flags::from((0b0000_0010, 0));
        let nt = Flags::from((0, 0));
        assert!(t.is_truncated());
        assert!(!nt.is_truncated());
    }

    #[test]
    fn can_check_opcode() {
        let q = Flags::from((0, 0));
        let iq = Flags::from((0b0000_1000, 0));
        let s = Flags::from((0b0001_0000, 0));
        let r = Flags::from((0b0111_1000, 0));
        assert_eq!(q.opcode(), Opcode::QUERY);
        assert_eq!(iq.opcode(), Opcode::IQUERY);
        assert_eq!(s.opcode(), Opcode::STATUS);
        assert_eq!(r.opcode(), Opcode::RESERVED);
    }
}
