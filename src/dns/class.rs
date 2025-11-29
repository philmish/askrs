use crate::dns::error::DnsError;

/*
Record Classes
IN              1 the Internet
CS              2 the CSNET class (Obsolete - used only for examples in
                some obsolete RFCs)
CH              3 the CHAOS class
HS              4 Hesiod [Dyer 87]
*               255 any class
*/
#[derive(Debug, Clone, Copy)]
pub enum RecordClass {
    IN,
    CS,
    CH,
    HS,
    ANY,
}

#[derive(Debug, Clone, Copy)]
pub struct Class(RecordClass);

impl TryFrom<RecordClass> for Class {
    type Error = DnsError;

    fn try_from(value: RecordClass) -> Result<Self, Self::Error> {
        match value {
            RecordClass::ANY => Err(DnsError::InvalidClass(value.into())),
            _ => Ok(Self(value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QClass(RecordClass);

impl Into<u16> for QClass {
    fn into(self) -> u16 {
        self.0.into()
    }
}

impl From<RecordClass> for QClass {
    fn from(value: RecordClass) -> Self {
        Self(value)
    }
}

impl TryFrom<u16> for RecordClass {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RecordClass::IN),
            2 => Ok(RecordClass::CS),
            3 => Ok(RecordClass::CH),
            4 => Ok(RecordClass::HS),
            255 => Ok(RecordClass::ANY),
            _ => Err(()),
        }
    }
}

impl Into<u16> for RecordClass {
    fn into(self) -> u16 {
        match self {
            RecordClass::IN => 1,
            RecordClass::CS => 2,
            RecordClass::CH => 3,
            RecordClass::HS => 4,
            RecordClass::ANY => 255,
        }
    }
}
