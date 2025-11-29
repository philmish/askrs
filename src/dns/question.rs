use byters::{BigEndian, ReadsIntoBytes};

use crate::dns::{
    class::{Class, QClass, RecordClass},
    name::{Label, Name, NameError},
    r#type::{QType, RecordType},
};

pub struct Question<'a> {
    name: Name<'a>,
    qtype: QType,
    qclass: QClass,
}

impl<'a> Question<'a> {
    pub fn a_type_record(name: Name<'a>) -> Self {
        Self {
            name,
            qtype: QType::from(RecordType::A),
            qclass: QClass::from(RecordClass::IN),
        }
    }

    pub fn try_to_bytes(&self) -> Result<Vec<u8>, NameError> {
        let name_labels: Vec<Label> = self.name.try_into()?;
        let mut name_bytes: Vec<u8> = name_labels
            .iter()
            .map(|l| l.as_bytes())
            .collect::<Vec<&[u8]>>()
            .concat();
        println!(
            "got {} bytes from {} labels",
            name_bytes.len(),
            name_labels.len()
        );
        let qt_bytes = BigEndian::read_from_u16(self.qtype.into()).to_vec();
        let qc_bytes = BigEndian::read_from_u16(self.qclass.into()).to_vec();
        Ok([name_bytes, qt_bytes, qc_bytes].concat())
    }
}
