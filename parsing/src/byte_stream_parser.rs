use dns::{header::Header, name::Label};

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
        if offset < self.curr_offset && offset as usize <= self.stream.len() {
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
        let bytes: Vec<u8> = match self.pop_n_from_stream(12) {
           Ok(b) => b,
           Err(err) => panic!("Failed to parse dns headers from byte stream {}", err),
        };
        return Ok(Header::from_bytes(bytes));
    }

    pub fn parse_question_labels(&mut self) -> Result<Vec<Label>, String> {
        let mut labels: Vec<Label> = vec![];
        let mut complet = false;
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
        } else {
            return Ok(labels);
        }
    }
}

#[cfg(test)]
mod tests {
    use dns::header::Header;

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
    fn test_parse_labels() {
        let data: Vec<u8> = vec![
            0x06, 0x67, 0x6F, 0x6F,
            0x67, 0x6C, 0x65, 0x03,
            0x63, 0x6F, 0x6D, 0x00,
        ];
        let mut parser = ByteStreamParser::new(&data);
        let labels = parser.parse_question_labels().unwrap();
        assert_eq!(parser.remaining_stream_len(), 0);
        assert_eq!(labels.len(), 2);
        assert_eq!(parser.curr_offset, 12);
    }
}

