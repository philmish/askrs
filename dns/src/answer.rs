use utility::{Row, Blob};

use crate::{name::Name, record::{RecordType, ARecord}};


trait ParsableRecord {
    fn read_r_data(data: Vec<u8>, offset: Option<u8>) -> Self;
    fn print_r_data(&self);
}

impl ParsableRecord for Name {
    fn print_r_data(&self) {
        println!("Name: {}", self.get_string().unwrap_or("is unparseable".to_string()));
    }

    fn read_r_data(data: Vec<u8>, offset: Option<u8>) -> Self {
        let bytes = data.get_from_offset(offset.unwrap_or(0)).unwrap();
        return Name::from_bytes(bytes, 0);
    }
}

//TODO Find a better way to parse answer data for Address or CNAME
//TODO make length a u16 and ttl a u32 as they are used as such
//TODO find a way to use traits for a_data field
pub struct Answer {
    name: Name,
    r_type: RecordType,
    class: [u8;2],
    ttl: [u8;4],
    length: [u8;2],
    a_data: Vec<u8>,
}

impl Answer {

    fn ttl_as_u32(&self) -> u32 {
        ((self.ttl[0] as u32) << 24) +
        ((self.ttl[1] as u32) << 16) +
        ((self.ttl[2] as u32) << 8) +
        ((self.ttl[3] as u32) << 0)
    }
    
    pub fn print(&self) {
        println!("Name: {}", self.name.get_string().unwrap());
        println!("Type: {}", self.r_type.to_string());
        println!("Class: {}", self.class.as_u16());
        println!("TTL: {}", self.ttl_as_u32());
        println!("Length: {}", self.length.as_u16());
        match self.r_type.to_string().as_str() {
            "A" => ARecord::from_bytes(self.a_data.to_vec(), 0).print(),
            _ => println!("unparseable answer data.")
        };
    }

    pub fn from_bytes(data: Vec<u8>, offset: u8) -> Result<Self, String> {
        let mut name: Name = Name::from_bytes(data.to_vec(), offset);
        let stat_offset: u8 = offset + name.get_bytes_length();
        let mut stats: [u8;10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut pos: usize = 0;
        let mut it = data.to_vec()
            .get_from_offset(stat_offset)
            .unwrap()
            .into_iter();
        while pos < 10 {
           stats[pos] = it.next().unwrap();
           pos += 1;
        }
        let r_type = RecordType::from_bytes([stats[0], stats[1]]);
        let class: [u8;2] = [stats[2], stats[3]];
        let ttl: [u8;4] = [stats[4], stats[5], stats[6], stats[7]];
        let length: [u8;2] = [stats[8], stats[9]];
        if it.len() < length.as_u16() as usize {
            return Err(format!(
                    "Invalid data length of {} for address with length of {}",
                    it.len(),
                    length.as_u16()
                    )
                );
        }
        pos = 0;
        let mut a_data: Vec<u8> = vec![];
        while pos < length.as_u16() as usize {
            match it.next() {
                Some(byte) => a_data.push(byte),
                None => panic!("Pushing nil to A Record Address")
            };
            pos += 1;
        }
        if name.is_compressed() {
            let decompressed = name.decompress(data.to_vec()).unwrap();
            name = decompressed;
        }

        return Ok(Answer{
            name,
            r_type,
            class,
            ttl,
            length,
            a_data,
        });

    }
}

#[cfg(test)]
mod tests {
    use super::Answer;

    #[test]
    fn test_parse_answer() {
        let data: Vec<u8> = vec![
            6,
            103,
            111,
            111,
            103,
            108,
            101,
            3,
            99,
            111,
            109,
            0,
            0,
            1,
            0,
            1,
            0,
            0,
            0,
            1,
            0,
            4,
            1,
            1,
            1,
            1,
        ];
        let answer = Answer::from_bytes(data, 0);
        assert!(match answer {
            Ok(_) => true,
            Err(_) => false,
        });
    }
}
