use dns::answer::Answer;
//use utility::Blob;
use dns::question::Question;
use dns::header::{self, Header};
use dns::record::RecordType;
use utility::Blob;

pub struct Query {
    header: header::Header,
    question: Question,
}

impl Query {

    fn new(domain: String, r_type: RecordType) -> Self {
        let header = Header::new_query(None);
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

    pub fn new_query(&self, domain: String, r_type: String) -> Query {
        return Query::new(domain, RecordType::from_string(r_type));
    }

    pub fn parse_header(&self, data: Vec<u8>) -> Header {
        return Header::from_bytes(data);
    }

    pub fn parse_question(&self, data: Vec<u8>) -> Result<Question, &str> {
        let chunk: Vec<u8> = data.get_from_offset(12).unwrap();
        return Question::from_bytes(chunk);
    }

    //TODO make it possible to parse multiple answer records
    pub fn parse_answer(&self, data: Vec<u8>, offset: u8) -> Result<Answer, String> {
        return Answer::from_bytes(data, offset);
    }

    pub fn parse_answers(&self, data: Vec<u8>, start_offset: u8, r_count: usize) -> Result<Vec<Answer>, String> {
        return Answer::multiple_from_bytes(data, start_offset, r_count);
    }
}
