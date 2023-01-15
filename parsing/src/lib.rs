use dns::answer::Answer;
use dns::question::Question;
use dns::header::{self, Header};
use dns::record::RecordType;

pub mod byte_stream_parser;

pub struct Query {
    header: header::Header,
    question: Question,
}

impl Query {

    pub fn new(domain: String, r_type: RecordType, rd: bool) -> Self {
        let header = Header::new_query(Some(rd));
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

    pub fn print(&self, verbose: bool) {
        self.header.print(verbose);
        self.question.print(verbose);
    }
}

pub struct Response {
    bytes: Vec<u8>,
    header: dns::header::Header,
    question: dns::question::Question,
    answers: Vec<dns::answer::Answer>,
}

impl Response {

    pub fn new(bytes: Vec<u8>, header: Header, question: Question, answers: Vec<Answer>) -> Self {
        return Self { bytes, header, question, answers }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        return self.bytes.to_vec();
    }

    pub fn print(&self, verbose: bool) {
        self.header.print(verbose);
        self.question.print(verbose);
        println!("Answer Records: {}", self.header.an_count());
        println!("NS Records: {}", self.header.ns_count());
        println!("Additional Records: {}", self.header.ar_count());
        for an in self.answers.to_vec().iter() {
            an.print(self.get_bytes());
        }
    }
}
