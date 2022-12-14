use utility::{Row, Blob};

use crate::{name::Name, record::{RecordType, ARecord, AAAARecord, CNAMERecord, MXRecord, NSRecord}};

//TODO make length a u16 and ttl a u32 as they are used as such
//TODO find a way to use generics and traits for a_data field
pub struct Answer {
    name: Name,
    r_type: RecordType,
    class: [u8;2],
    ttl: [u8;4],
    length: [u8;2],
    a_data: Vec<u8>,
}

impl Clone for Answer {
    fn clone(&self) -> Self {
        return Answer {
            name: self.name.clone(),
            r_type: self.r_type.clone(),
            class: self.class.clone(),
            ttl: self.ttl.clone(),
            length: self.length.clone(),
            a_data: self.a_data.to_vec(),
        };
    }
}

impl Answer {

    fn ttl_as_u32(&self) -> u32 {
        ((self.ttl[0] as u32) << 24) +
        ((self.ttl[1] as u32) << 16) +
        ((self.ttl[2] as u32) << 8) +
        ((self.ttl[3] as u32) << 0)
    }

    fn length(&self) -> u8 {
        return 12 + self.length.as_u16() as u8;
    }
    
    pub fn print(&self, src: Vec<u8>) {
        println!("---------------------");
        println!("\tName: {}", self.name.get_string().unwrap());
        println!("\tType: {}", self.r_type.to_string());
        println!("\tClass: {}", self.class.as_u16());
        println!("\tTTL: {}", self.ttl_as_u32());
        println!("\tLength: {}", self.length.as_u16());
        match self.r_type.to_string().as_str() {
            "A" => ARecord::from_bytes(self.a_data.to_vec(), 0).print(),
            "AAAA" => AAAARecord::from_bytes(self.a_data.to_vec(), 0).print(),
            "CNAME" => CNAMERecord::from_bytes(self.a_data.to_vec(), src, 0).print(),
            "MX" => MXRecord::from_bytes(self.a_data.to_vec(), src, 0).print(),
            "NS" => NSRecord::from_bytes(self.a_data.to_vec(), src, 0).print(),
            _ => println!("\tunparseable answer data.")
        };
        println!("---------------------");
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

    pub fn multiple_from_bytes(data: Vec<u8>, start_offset: u8, r_count: usize) -> Result<Vec<Self>, String> {
        let mut curr_offset: u8 = start_offset; 
        let mut res: Vec<Self> = vec![];
        let mut answer: Self;
        for _n in 0..r_count {
           answer = Self::from_bytes(data.to_vec(), curr_offset).unwrap();
           curr_offset += answer.length();
           res.push(answer);
        }
        if res.len() == 0 {
            return Err(String::from("Something went wrong parsing the answer records. Could parse 0."));
        } else {
            return Ok(res);
        }
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
