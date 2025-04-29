/*
CLASS fields appear in resource records.  The following CLASS mnemonics
and values are defined:

IN              1 the Internet
CS              2 the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
CH              3 the CHAOS class
HS              4 Hesiod [Dyer 87]
*/

use crate::data::Row;
use std::fmt::Display;

pub(crate) enum QClass {
    IN,
    CS,
    CH,
    HS,
    UNKNOWN(u16),
}

impl Display for QClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IN => write!(f, "Internet"),
            Self::CS => write!(f, "CSNet (Obsolete)"),
            Self::CH => write!(f, "Chaos"),
            Self::HS => write!(f, "Hesiod"),
            Self::UNKNOWN(v) => write!(f, "Invalid Class {:#02X?}", v),
        }
    }
}

impl From<u16> for QClass {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            _ => Self::UNKNOWN(value),
        }
    }
}

impl From<Row> for QClass {
    fn from(value: Row) -> Self {
        Self::from(u16::from(value))
    }
}
