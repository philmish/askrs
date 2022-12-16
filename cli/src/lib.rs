use std::env;

pub mod socket;

use parsing::Parser;

pub struct Flags {
    domain: String,
    record: String,
}

impl Flags {

    pub fn parse() -> Result<Self, &'static str> {
        // TODO Figure out a better way to parse flags.
       let mut args = env::args(); 
       if args.len() < 2 {
          return Err("You need to provide a domain.");
       } else if args.len() == 2 {
           return Ok(Self { 
               domain: args.nth(1).expect("Missing Domain"),
               record: "A".to_string() 
           });
       } else {
           return Ok(Self { 
               domain: args.nth(1).expect("Missing Domain"),
               record: args.nth(1).expect("Missing Record Type")
           });
       }
    }

    pub fn get_domain(&self) -> String {
        return self.domain.to_string();
    }
    
    pub fn get_r_type(&self) -> String {
        return self.record.to_string();
    }

    pub fn print(&self) {
        println!("Domain: {}", self.domain);
        println!("Record Type: {}", self.record);

    }
}

pub struct CLI {
    flags: Flags,
    parser: Parser,
}

impl CLI {

    pub fn init() -> Self {
        let flags = Flags::parse().expect("Failed to read flags.");
        let parser = Parser{};
        return Self{
            flags,
            parser
        };
    }

    pub fn run(&self) {
        let q = self.parser.new_query(
            self.flags.get_domain(),
            self.flags.get_r_type()
            );
        let client = socket::UDPClient{};
        let msg = q.to_bytes();
        let a = client.send_and_recieve(msg, socket::DNSSocket::GOOGLE).unwrap();
        q.print();
        println!("----------------------");
        let resp_header = self.parser.parse_header(a.to_vec());
        resp_header.print();
        let question = self.parser.parse_question(a.to_vec()).unwrap();
        question.print();
        let answer = self.parser.parse_answer(a.to_vec(), 12 + question.length()).unwrap();
        answer.print();
    }
}
