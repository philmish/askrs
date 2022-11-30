//use utility::Blob;
use dns::question::Question;
use dns::header::{self, Header};
use dns::record::RecordType;

pub struct Query {
    header: header::Header,
    question: Question,
}

impl Query {

    fn new(domain: String, r_type: RecordType) -> Self {
        let header = Header::new_query(None, None);
        let question = Question::new(domain, Some(r_type), None);
        return Self{
            header,
            question
        };
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        res.extend(self.header.to_bytes());
        res.extend(self.question.to_bytes());

        return res;
    }

    pub fn print(&self) {
        self.header.print();
        self.question.print();
    }
}

pub struct Parser;

impl Parser {

    pub fn new_query(&self, domain: String, r_type: String) -> Query{
        return Query::new(domain, RecordType::from_string(r_type));
    }
}
