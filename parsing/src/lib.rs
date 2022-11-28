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
