use header::Header;
use name::DomainName;
use record::Record;

pub mod header;
pub mod name;
pub mod record;

pub struct Question {
    name: DomainName,
    qtype: u16,
    qclass: u16,
}

pub struct Message {
    header: Header,
    question: Question,
    answers: Option<Vec<Record>>,
    authority: Option<Vec<Record>>,
    additional: Option<Vec<Record>>,
}
