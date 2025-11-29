use crate::dns::{error::DnsError, header::MessageHeader, question::Question, record::RecordData};

pub struct Message<'a> {
    header: MessageHeader,
    question: Question<'a>,
    answers: Option<Vec<RecordData<'a>>>,
    authority: Option<Vec<RecordData<'a>>>,
    additional: Option<Vec<RecordData<'a>>>,
}
