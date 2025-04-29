use crate::bits::HasBits;

#[derive(Clone, Copy)]
pub(crate) struct Row(u8, u8);

impl From<Row> for u16 {
    fn from(value: Row) -> Self {
        ((value.0 as u16) << 8) | value.1 as u16
    }
}

impl Row {
    pub(crate) fn new(head: u8, tail: u8) -> Self {
        return Self(head, tail);
    }
    pub(crate) fn head(&self) -> u8 {
        self.0
    }

    pub(crate) fn tail(&self) -> u8 {
        self.1
    }

    pub(crate) fn head_set_bits(&mut self, map: u8) {
        self.0.set_bits(map);
    }

    pub(crate) fn head_unset_bits(&mut self, map: u8) {
        self.0.unset_bits(map);
    }

    pub(crate) fn tail_set_bits(&mut self, map: u8) {
        self.1.set_bits(map);
    }

    pub(crate) fn tail_unset_bits(&mut self, map: u8) {
        self.1.unset_bits(map);
    }

    pub(crate) fn bit_is_set(&self, pos: usize) -> bool {
        if pos < 8 {
            return self.0.bit_is_set(pos);
        } else if pos < 16 {
            return self.1.bit_is_set(pos);
        }
        return false;
    }
}

pub(crate) struct Label {
    length: usize,
    content: [u8; 63],
}

impl Label {
    pub(crate) fn zero_label() -> Self {
        Label {
            length: 0,
            content: [0u8; 63],
        }
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        if self.length == 0 {
            return vec![0];
        }
        let mut buf: Vec<u8> = Vec::with_capacity(self.length + 1);
        buf.push(self.length as u8);
        for b in self.content.iter() {
            buf.push(b.clone());
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
                let len = value.bytes().len();
                let mut buf = [0u8; 63];
                for (i, ch) in value.chars().enumerate().take(63) {
                    buf[i] = ch as u8
                }
                return Ok(Label {
                    length: len,
                    content: buf,
                });
            }
        }
    }
}

pub(crate) struct Name(Vec<Label>);

// TODO: Research if Btreemap or HashMap is more useful here.
//       This will be used to resolve compressed names from a response.
//       It will be used to map labels to offsets.
// pub(crate) struct LabelMap();
