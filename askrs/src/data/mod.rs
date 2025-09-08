use crate::bits::HasBits;

pub(crate) mod label;

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

pub(crate) struct Name(Vec<label::Label>);

// TODO: Research if Btreemap or HashMap is more useful here.
//       This will be used to resolve compressed names from a response.
//       It will be used to map labels to offsets.
// pub(crate) struct La{belMap();
