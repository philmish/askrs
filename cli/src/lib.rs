use std::env;
use parsing::{Parser, Query};

pub mod socket;


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

    fn send_query(&self, q: Query, srv: Option<socket::DNSSocket>) -> Vec<u8> {
        let client = socket::UDPClient{};
        let msg = q.to_bytes();
        let a = client.send_and_recieve(
            msg,
            srv.unwrap_or(socket::DNSSocket::GOOGLE)
            )
            .unwrap();
        q.print();
        return a;
    }

    pub fn run(&self) {
        let q = self.parser.new_query(
            self.flags.get_domain(),
            self.flags.get_r_type()
            );
        let a = self.send_query(q, None);
        println!("----------------------");
        let resp_header = self.parser.parse_header(a.to_vec());
        resp_header.print();
        let question = self.parser.parse_question(a.to_vec()).unwrap();
        question.print();
        if resp_header.rcode().is_err() {
            resp_header.rcode().print();
            return;
        }
        let offset: u8 = 12 + question.length();
        if resp_header.an_count() == 1 {
            let answer = self.parser.parse_answer(a.to_vec(), offset).unwrap();
            answer.print();
            return;
        } else if resp_header.an_count() > 1 {
            let answers = self.parser
                              .parse_answers(a.to_vec(), offset, resp_header.an_count() as usize)
                              .unwrap();
            for an in answers.iter() {
               an.print(); 
            }
        } else {
            println!("No answers received.");
        }
    }
}
