use crate::data::label::Label;

use super::{qclass::QClass, qtypes::Qtype};

pub(crate) struct DnsQuestion {
    name: Vec<Label>,
    qtype: Qtype,
    class: QClass,
}

impl DnsQuestion {
    pub fn new(name: Vec<Label>, qtype: Qtype, class: QClass) -> Self {
        DnsQuestion { name, qtype, class }
    }
}
