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
        // TODO Implement socket client to send question
        // TODO Implement answer parsing        
        let q = self.parser.new_query(
            self.flags.get_domain(),
            self.flags.get_r_type()
            );
        let client = socket::UDPClient{};
        let msg = q.to_bytes();
        let a = client.send_and_recieve(msg, socket::DNSSocket::GOOGLE).unwrap();
        println!("{}", a.len());
        q.print();
    }
}
