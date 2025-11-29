use byters::{BigEndian, ReadsFromBytes, ReadsIntoBytes};

use crate::dns::error::DnsError;

#[derive(Debug, Clone, Copy)]
pub struct Name<'a>(&'a str);

#[derive(Debug)]
pub enum Label {
    Fragment(Vec<u8>),
    End,
}

impl<'a> TryFrom<&'a str> for Name<'a> {
    type Error = NameError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let l = value.len();
        if l > 255 {
            return Err(NameError::NameToLong(l));
        }
        if l == 0 {
            return Err(NameError::EmptyName);
        }
        Ok(Self(value))
    }
}

impl<'a> TryInto<Vec<Label>> for Name<'a> {
    type Error = NameError;

    fn try_into(self) -> Result<Vec<Label>, Self::Error> {
        if self.0.len() > 255 {
            return Err(NameError::NameToLong(self.0.len()));
        }
        let mut out: Vec<Label> = vec![];
        for fragment in self.0.split_terminator('.') {
            let l = Label::try_from(fragment)?;
            out.push(l);
        }
        out.push(Label::End);
        Ok(out)
    }
}

impl Label {
    pub fn byte_len(&self) -> usize {
        match self {
            Self::Fragment(v) => v[0] as usize,
            Self::End => 1,
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Fragment(v) => v.as_slice(),
            Self::End => &[0],
        }
    }
}

impl TryFrom<&str> for Label {
    type Error = NameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let l = value.len();
        if l > 63 {
            return Err(NameError::LabelToLong(l));
        }
        if l == 0 {
            return Ok(Self::End);
        }
        let mut v: Vec<u8> = Vec::with_capacity(l + 1);
        v.push(l as u8);
        v.extend_from_slice(value.as_bytes());
        Ok(Label::Fragment(v))
    }
}

#[derive(Debug)]
pub enum NameError {
    EmptyName,
    NameToLong(usize),
    LabelToLong(usize),
}
