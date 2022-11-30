use utility::{Row, Blob};

use crate::{name, record::RecordType};

pub enum QClass {
    INET,
}

impl QClass {
    pub fn get_bytes(class: QClass) -> [u8;2] {
        return match class {
            QClass::INET => [0b0000_0000, 0b0000_0001],
        };
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        let b: [u8;2] = data.to_vec().try_into().unwrap();
        return match b.as_u16() {
            1 => QClass::INET,
            _ => QClass::INET,
        };
    }

    pub fn to_string(&self) -> String {
        match self {
            QClass::INET => "INET".to_string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            QClass::INET => vec![0b0000_0000, 0b0000_0001]
        }
    }
}

pub struct Question {
    q_name: name::Name,
    q_type: RecordType,
    q_class: QClass,
}

impl Question {
    pub fn new(name: String, q_type: Option<RecordType>, q_class: Option<QClass>) -> Self {
        return Question{
            q_name: name::Name::from_string(name).unwrap(),
            q_type: q_type.unwrap_or(RecordType::A),
            q_class: q_class.unwrap_or(QClass::INET),
        };
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self, &'static str> {
       let mut name = name::Name::from_bytes(data.to_vec()) ;
       if name.is_compressed() {
           name = name.decompress(data.to_vec()).unwrap();
       }
       let data = data.get_from_offset(name.get_bytes_length()).unwrap();
       if data.len() < 4 {
           return Err("Question must end with at least 4 bytes.");
       }
       let q_type: [u8;2] = [data[0], data[1]];
       let q_class: QClass = QClass::from_bytes(vec![data[2], data[3]]);
       return Ok(
           Question {
               q_name: name,
               q_type: RecordType::from_bytes(q_type),
               q_class 
           });
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        res.extend(self.q_name.get_bytes());
        res.extend(self.q_type.to_bytes());
        res.extend(self.q_class.to_bytes());
        return res;
    }

    pub fn print(&self) {
        println!("Domain: {}", self.q_name.get_string().unwrap());
        println!("Question Type: {}", self.q_type.to_string());
        println!("Question Class: {}", self.q_class.to_string());
    }
}
