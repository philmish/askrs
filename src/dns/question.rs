use byters::{BigEndian, ReadsIntoBytes};

use crate::dns::{
    class::QClass,
    name::{Label, Name, NameError},
    r#type::QType,
};

pub struct Question<'a> {
    name: Name<'a>,
    qtype: QType,
    qclass: QClass,
}

impl Question<'_> {
    pub fn try_to_bytes(&self) -> Result<Vec<u8>, NameError> {
        let name_labels: Vec<Label> = self.name.try_into()?;
        let mut name_bytes: Vec<u8> = name_labels
            .iter()
            .map(|l| l.as_bytes())
            .collect::<Vec<&[u8]>>()
            .concat();
        let qt_bytes = BigEndian::read_from_u16(self.qtype.into()).to_vec();
        let qc_bytes = BigEndian::read_from_u16(self.qclass.into()).to_vec();
        Ok([name_bytes, qt_bytes, qc_bytes].concat())
    }
}
