use crate::data::Row;
use std::fmt::Display;

const PTR_MASK: u8 = 0b1100_0000;
const IS_PTR_MASK: u16 = 0xC000;
const PTR_OFFSET_MASK: u16 = 0x03FF;

#[derive(Clone)]
pub(crate) enum NameFragment {
    Zero,
    Pointer(u16),
    Label([u8; 64]),
}

impl NameFragment {
    pub(crate) fn byte_len(&self) -> usize {
        match self {
            Self::Zero => 1,
            Self::Pointer(_) => 2,
            Self::Label(v) => v[0] as usize,
        }
    }

    pub(crate) fn offset(&self) -> usize {
        match self {
            Self::Pointer(v) => (v & PTR_OFFSET_MASK) as usize,
            _ => 0,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::Zero => vec![0],
            Self::Pointer(v) => {
                let mut buf = Vec::with_capacity(2);
                buf.push((v >> 8 & 0x00FF) as u8);
                buf.push((v & 0x00FF) as u8);
                buf
            }
            Self::Label(v) => {
                let mut buf = Vec::with_capacity(v[0] as usize);
                for b in 1..=v[0] as usize {
                    buf.push(v[b]);
                }
                buf
            }
        }
    }
}

pub struct DnsName(Vec<NameFragment>);

impl DnsName {
    pub fn new(fragments: Vec<NameFragment>) -> Self {
        let mut inner: Vec<NameFragment> = Vec::with_capacity(fragments.len());
        for f in fragments.iter() {
            match f {
                NameFragment::Zero => {
                    inner.push(f.clone());
                    break;
                }
                NameFragment::Label(_) => {
                    inner.push(f.clone());
                    continue;
                }
                NameFragment::Pointer(_) => {
                    inner.push(f.clone());
                    break;
                }
            }
        }
        DnsName(inner)
    }
}

impl TryFrom<String> for NameFragment {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "" => Ok(NameFragment::Zero),
            _ => {
                if !value.is_ascii() {
                    return Err("Label must only contain ascii symbols");
                }
                let len = value.bytes().len() as u8;
                if len > 63 {
                    return Err("Label string to long");
                }
                let mut buf = [0u8; 64];
                buf[0] = len;
                for (i, ch) in value.chars().enumerate().take(63) {
                    buf[i + 1] = ch as u8
                }
                return Ok(NameFragment::Label(buf));
            }
        }
    }
}

pub(crate) struct Label {
    content: [u8; 64],
}

impl Label {
    pub(crate) fn new_zero_label() -> Self {
        Label { content: [0u8; 64] }
    }

    pub(crate) fn is_zero_label(&self) -> bool {
        self.content[0] == 0
    }

    pub(crate) fn is_pointer(&self) -> bool {
        (PTR_MASK & self.content[0]) == PTR_MASK
    }

    pub(crate) fn len(&self) -> usize {
        self.content[0] as usize
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        if self.len() == 0 {
            return vec![0];
        }
        if self.is_pointer() {
            return vec![self.content[0], self.content[1]];
        }
        let len = self.len();
        let mut buf: Vec<u8> = Vec::with_capacity(len);
        buf.push(self.content[0]);
        for b in 1..=len {
            buf.push(self.content[b]);
        }
        return buf;
    }
}

impl TryFrom<String> for Label {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "" => Ok(Label::zero_label()),
            _ => {
                if !value.is_ascii() {
                    return Err(String::from("Label must only contain ascii symbols"));
                }
                let len = value.bytes().len() as u8;
                if len > 63 {
                    return Err(String::from("Label string to long"));
                }
                let mut buf = [0u8; 64];
                buf[0] = len;
                for (i, ch) in value.chars().enumerate().take(63) {
                    buf[i + 1] = ch as u8
                }
                return Ok(Label { content: buf });
            }
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_pointer() {
            let offset: u16 = Row::new(self.content[0], self.content[1]).into();
            return write!(f, "Label at offset {}", offset);
        }
        let mut str_buf = Vec::with_capacity(self.len());
        for i in 1..=self.content[0] as usize {
            str_buf.push(self.content[i]);
        }
        let out = String::from_utf8_lossy(str_buf.as_slice());
        write!(f, "{}", out)
    }
}
