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

pub struct Response {
    bytes: Vec<u8>,
    header: dns::header::Header,
    question: dns::question::Question,
    answers: Vec<dns::answer::Answer>,
}

impl Response {

    pub fn from_bytes(data: Vec<u8>, parser: Parser) -> Result<Self, String> {
        let resp_header = parser.parse_header(data.to_vec());
        let question = parser.parse_question(data.to_vec()).unwrap();
        if resp_header.rcode().is_err() {
            resp_header.rcode().print();
            return Err("Error response recieved".to_string());
        }
        let offset: u8 = 12 + question.length();
        if resp_header.an_count() > 0 {
            let answers = parser.parse_answers(data.to_vec(), offset, resp_header.an_count() as usize)
                                .unwrap();
            let resp: Response = Response { 
                bytes: data.to_vec(), 
                header: resp_header, 
                question, 
                answers 
            };
            return Ok(resp);
        } else {
            let resp: Response = Response { 
                bytes: data.to_vec(), 
                header: resp_header, 
                question, 
                answers: vec![]
            };
            return Ok(resp);
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        return self.bytes.to_vec();
    }

    pub fn print(&self) {
        self.header.print();
        self.question.print();
        for an in self.answers.to_vec().iter() {
            an.print();
        }
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

    pub fn parse_answer(&self, data: Vec<u8>, offset: u8) -> Result<Answer, String> {
        return Answer::from_bytes(data, offset);
    }

    pub fn parse_answers(&self, data: Vec<u8>, start_offset: u8, r_count: usize) -> Result<Vec<Answer>, String> {
        return Answer::multiple_from_bytes(data, start_offset, r_count);
    }
}
