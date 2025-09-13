use std::fmt;

use utility::Row;

use crate::{name, record::RecordType};

pub enum QClass {
    INET,
}

impl QClass {
    pub fn get_bytes(class: QClass) -> [u8; 2] {
        return match class {
            QClass::INET => [0b0000_0000, 0b0000_0001],
        };
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        let b: [u8; 2] = data.to_vec().try_into().unwrap();
        return match b.as_u16() {
            1 => QClass::INET,
            _ => QClass::INET,
        };
    }

    pub fn from_row(data: [u8; 2]) -> Self {
        match data.as_u16() {
            1 => QClass::INET,
            _ => QClass::INET,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            QClass::INET => "INET".to_string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            QClass::INET => vec![0b0000_0000, 0b0000_0001],
        }
    }
}

pub struct Question {
    q_name: name::Name,
    q_type: RecordType,
    q_class: QClass,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "URI: {}\nQType: {}\nQClass: {}",
            self.q_name.get_string().unwrap(),
            self.q_type.to_string(),
            self.q_class.to_string()
        )
    }
}

impl Question {
    pub fn init(q_name: name::Name, q_type: RecordType, q_class: QClass) -> Self {
        return Self {
            q_name,
            q_type,
            q_class,
        };
    }

    pub fn new(name: String, q_type: Option<RecordType>, q_class: Option<QClass>) -> Self {
        return Question {
            q_name: name::Name::from_string(name).unwrap(),
            q_type: q_type.unwrap_or(RecordType::A),
            q_class: q_class.unwrap_or(QClass::INET),
        };
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        res.extend(self.q_name.get_bytes());
        res.extend(self.q_type.to_bytes());
        res.extend(self.q_class.to_bytes());
        return res;
    }

    pub fn print(&self) {
        println!("{}", self)
    }

    pub fn length(&self) -> u8 {
        return self.q_name.get_bytes_length() + 4;
    }
}
