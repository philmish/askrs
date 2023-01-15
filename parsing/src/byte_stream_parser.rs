use dns::{
    header::Header,
    name::{Label, Name},
    question::{Question, QClass},
    record::RecordType,
    answer::Answer
};
use utility::Row;

use crate::Response;

pub struct ByteStream<'slice> {
    data: &'slice [u8],
}

impl Clone for ByteStream<'_> {
    fn clone(&self) -> Self {
        return ByteStream {
           data: self.data.clone() ,
        };
    }
}

impl<'slice> ByteStream<'slice> {
    pub fn new(bytes: &'slice Vec<u8>) -> ByteStream<'slice> {
        let stream: &[u8] = bytes.as_slice();
        return ByteStream{data: stream};
    }

    pub fn copy_bytes(&self) -> Vec<u8> {
        return self.data.to_vec();
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn get_iter(&self) -> std::slice::Iter<'slice, u8> {
        self.data.iter()
    }
}

pub struct ByteStreamParser<'slice> {
    data: ByteStream<'slice>,
    stream: std::slice::Iter<'slice, u8>,
    curr_offset: u8,
}

impl<'slice> ByteStreamParser<'slice> {

    pub fn new(data: &'slice Vec<u8>) -> Self {
        let stream = ByteStream::new(&data);
        return Self{
            data: stream.clone(),
            stream: stream.get_iter(),
            curr_offset: 0,
        };
    }

    pub fn reset_stream(&mut self) {
        self.stream = self.data.get_iter();
        self.curr_offset = 0;
    }


    pub fn pop_n_from_stream(&mut self, n: usize) -> Result<Vec<u8>, String> {
        if self.remaining_stream_len() < n {
            return Err("Stream to short to take items".to_string());
        }
        if self.remaining_stream_len() == 0 {
            self.reset_stream();
        }
        let mut res: Vec<u8> = vec![];
        let mut taken: u8 = 0;
        let mut cursor: u8;
        while (taken as usize) < n {
            cursor = *self.stream.next().unwrap();
            res.push(cursor);
            taken += 1;
        }
        return Ok(res);
    }
    
    pub fn set_stream_to_offset(&mut self, offset: u8) {
        if offset > self.curr_offset && offset as usize <= self.stream.len() {
            let n: u8 = offset - self.curr_offset;
            let _ = self.pop_n_from_stream(n as usize).unwrap();
        } else {
            self.reset_stream();
            let n: u8 = offset - self.curr_offset;
            let _ = self.pop_n_from_stream(n as usize);
        }
    }

    pub fn take_stream_slice(&mut self, size: u8) -> Result<Vec<u8>, String> {
        if size as usize > self.remaining_stream_len() {
            return Err("Slice size is bigger than the remaining length of the stream.".to_string());
        }
        let mut curr_size: u8 = 0;
        let mut res: Vec<u8> = vec![];
        let mut cursor: u8;
        while curr_size < size {
            cursor = *self.stream.next().unwrap();
            res.push(cursor);
            curr_size += 1;
            self.curr_offset += 1;
        }
        return Ok(res);
    }

    pub fn remaining_stream_len(&self) -> usize {
        return self.stream.len();
    } 

    pub fn parse_dns_header(&mut self) -> Result<Header, String> {
        if self.curr_offset != 0 {
            self.reset_stream();
        }
        match self.pop_n_from_stream(12) {
           Ok(b) => Ok(Header::from_bytes(b)),
           Err(err) =>  Err(err),
        }
    }

    pub fn parse_name(&mut self) -> Result<Name, String> {
        let mut labels: Vec<Label> = vec![];
        let mut complet = false;
        let mut compressed: bool = false;
        let mut len: u8;
        while !complet {
            if self.remaining_stream_len() == 0{
                complet = true;
                continue;
            }
            len = *self.stream.next().unwrap();
            self.curr_offset += 1;
            if len == 0 {
                complet = true;
                continue;
            } else if len == 192 {
                let offset = self.stream.next().unwrap();
                self.curr_offset += 1;
                labels.push(
                    Label::new(len, true, *offset, vec![192, *offset])
                );
                compressed = true;
                complet = true;
                continue;
            } else {
                let mut b: Vec<u8> = vec![];
                let mut counter: u8 = 0;
                while counter < len {
                    b.push(*self.stream.next().unwrap());
                    counter += 1;
                    self.curr_offset += 1;
                }
                labels.push(
                    Label::new(len, false, 0, b)
                );
                continue;
            }
        }
        if labels.len() == 0 {
            return Err("Something went wrong parsing the labels.".to_string());
        }
        let name = Name::new(labels, compressed);
        return Ok(name);
    }

    pub fn take_row(&mut self) -> Result<[u8;2], String> {
        if self.remaining_stream_len() == 0 {
            self.reset_stream();
        }
        let mut buf: [u8;2] = [0, 0];
        for n in 0..2 {
            buf[n] = *self.stream.next().unwrap();
            self.curr_offset += 1;
        }
        return Ok(buf);
    }

    pub fn parse_question(&mut self) -> Result<Question, String> {
        let name = self.parse_name().unwrap();
        let qtype = match self.take_row() {
            Ok(b) => RecordType::from_bytes(b),
            Err(err) => panic!("Failed to parse Question from byte stream: {}", err),
        };
        let qclass = match self.take_row() {
            Ok(qc) => QClass::from_row(qc),
            Err(err) => panic!("Failed to parse Question from byte stream: {}", err),
        };
        return Ok(Question::init(name, qtype, qclass));
    }

    pub fn parse_answer(&mut self) -> Result<Answer, String> {
        let mut name = match self.parse_name() {
           Ok(n) => n,
           Err(err) => panic!("Failed to parse Name in answer: {}", err),
        };
        if name.is_compressed() {
            name = name.decompress(self.data.copy_bytes()).unwrap();
        }
        let r_type = RecordType::from_bytes(self.take_row().unwrap());
        let class = self.take_row().unwrap();
        let mut ttl: [u8;4] = [0, 0, 0, 0];
        for n in 0..4 {
            ttl[n] = *self.stream.next().unwrap();
            self.curr_offset += 1;
        }
        let length: [u8;2] = self.take_row().unwrap();
        let a_data: Vec<u8> = self.pop_n_from_stream(length.as_u16() as usize).unwrap();
        return Ok(
            Answer::new(
                name,
                r_type,
                class,
                ttl,
                length,
                a_data,
            )
        )
    }

    pub fn parse_answers(&mut self, r_count: u8) -> Result<Vec<Answer>, String> {
        let mut res: Vec<Answer> = vec![];
        let mut answer: Answer;
        for _n in 0..r_count {
           answer = self.parse_answer().unwrap();
           res.push(answer);
        }
        if res.len() == 0 {
            return Err(String::from("Something went wrong parsing the answer records. Could parse 0."));
        } else {
            return Ok(res);
        }
    }

    pub fn parse_response(&mut self) -> Result<Response, String> {
        if self.curr_offset != 0 {
            self.reset_stream();
        }
        let header = self.parse_dns_header().unwrap();
        let question = self.parse_question().unwrap();
        let mut records: Vec<Answer> = vec![];
        for _ in 0..header.an_count() {
            records.push(self.parse_answer().unwrap());
        }
        let mut ns_records: Vec<Answer> = vec![];
        if header.ns_count() > 0 {
            for _ in 0..header.ns_count() {
                ns_records.push(self.parse_answer().unwrap());
            }
        }
        let mut answers: Vec<Answer> = vec![];
        answers.extend(records);
        answers.extend(ns_records);
        return Ok(
            Response::new(
                self.data.copy_bytes(),
                header,
                question,
                answers
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use dns::header::Header;

    use crate::Response;

    use super::ByteStreamParser;

    #[test]
    fn test_take_slice() {
        let data: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12];
        let data_len = data.len();
        let mut parser = ByteStreamParser::new(&data);
        assert_eq!(parser.remaining_stream_len(), data_len);

        let sl = parser.take_stream_slice(3).unwrap();
        assert_eq!(sl, vec![1,2,3]);
        assert_eq!(parser.remaining_stream_len(), data_len - 3);
    }

    #[test]
    fn test_pop_n() {
        let data: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12];
        let mut parser = ByteStreamParser::new(&data);
        let mut taken = parser.pop_n_from_stream(3).unwrap();
        assert_eq!(taken, vec![1,2,3]);
        assert_eq!(parser.remaining_stream_len(), 9);
        taken = parser.pop_n_from_stream(3).unwrap();
        assert_eq!(taken, vec![4,5,6]);
        assert_eq!(parser.remaining_stream_len(), 6);
    }

    #[test]
    fn test_set_stream_to_offset() {
        let data: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12];
        let mut parser = ByteStreamParser::new(&data);
        parser.set_stream_to_offset(3);
        assert_eq!(9, parser.remaining_stream_len());
    }

    #[test]
    fn test_reset_stream() {
        let data: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12];
        let mut parser = ByteStreamParser::new(&data);
        parser.set_stream_to_offset(3);
        assert_eq!(9, parser.remaining_stream_len());
        parser.reset_stream();
        assert_eq!(12, parser.remaining_stream_len());
    }

    #[test]
    fn test_parse_header() {
        let data: Vec<u8> = vec![
            0xDE, 0xAD, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let mut parser = ByteStreamParser::new(&data);
        let header: Header = parser.parse_dns_header().unwrap();
        assert_eq!(parser.remaining_stream_len(), 0);
        assert_eq!(header.q_count(), 1);
    }

    #[test]
    fn test_parse_name() {
        let data: Vec<u8> = vec![
            0x06, 0x67, 0x6F, 0x6F,
            0x67, 0x6C, 0x65, 0x03,
            0x63, 0x6F, 0x6D, 0x00,
        ];
        let mut parser = ByteStreamParser::new(&data);
        let name = parser.parse_name().unwrap();
        assert_eq!(parser.remaining_stream_len(), 0);
        assert_eq!(name.get_string().unwrap(), "google.com".to_string());
        assert_eq!(name.is_compressed(), false);
        assert_eq!(parser.curr_offset, 12);
    }

    #[test]
    fn test_parse_answer() {
        let data: Vec<u8> = vec![
            6, 103, 111, 111,
            103, 108, 101, 3,
            99, 111, 109, 0,
            0, 1, 0, 1,
            0, 0, 0, 1,
            0, 4, 1, 1,
            1, 1,
        ];
        let mut parser = ByteStreamParser::new(&data);
        assert!(match parser.parse_answer() {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    #[test]
    fn test_parse_answers() {
        let data: Vec<u8> = vec![
            6, 103, 111, 111,
            103, 108, 101, 3,
            99, 111, 109, 0,
            0, 1, 0, 1,
            0, 0, 0, 1,
            0, 4, 1, 1,
            1, 1,
            6, 103, 111, 111,
            103, 108, 101, 3,
            99, 111, 109, 0,
            0, 1, 0, 1,
            0, 0, 0, 1,
            0, 4, 1, 1,
            1, 1,
            6, 103, 111, 111,
            103, 108, 101, 3,
            99, 111, 109, 0,
            0, 1, 0, 1,
            0, 0, 0, 1,
            0, 4, 1, 1,
            1, 1,
        ];

        let mut parser = ByteStreamParser::new(&data);
        assert!(match parser.parse_answers(3) {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    #[test]
    fn test_parse_respone() {
        let data: Vec<u8> = vec![
            0xDE, 0xAD, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00,
            0x06, 0x67, 0x6F, 0x6F,
            0x67, 0x6C, 0x65, 0x03,
            0x63, 0x6F, 0x6D, 0x00,
            0x00, 0x01, 0x00, 0x01,
            192, 12,0, 1, 
            0, 1, 0, 0, 
            0, 1,0, 4, 
            1, 1, 1, 1,
        ];
        let mut parser = ByteStreamParser::new(&data);
        let response: Response = parser.parse_response().unwrap();
        response.print(true);
    }
}

