pub trait Row {
    fn as_u16(&self) -> u16;
    fn start_set_bits(&mut self, byte: u8);
    fn start_unset_bits(&mut self, byte: u8);
    fn end_set_bits(&mut self, byte: u8);
    fn end_unset_bits(&mut self, byte: u8);
}

impl Row for [u8;2] {

    fn as_u16(&self) -> u16 {
        return ((self[0] as u16) << 8) | self[1] as u16;
    }

    fn start_set_bits(&mut self, byte: u8) {
        self[0] |= byte;
    }

    fn start_unset_bits(&mut self, byte: u8) {
        self[0] &= byte;
    }

    fn end_set_bits(&mut self, byte: u8) {
        self[1] |= byte;
    }

    fn end_unset_bits(&mut self, byte: u8) {
        self[1] &= byte;
    }
}

pub trait Blob {
   fn get_slice(&self, start: u16, end: u16) -> Result<Vec<u8>, &'static str>;
   fn get_from_offset(&self, start: u8) -> Result<Vec<u8>, &'static str>;
   fn to_socket_msg(&self) -> Result<&[u8], &'static str>;
}

impl Blob for Vec<u8> {

    fn get_slice(&self, start: u16, end: u16) -> Result<Self, &'static str> {
        if end < start || end as usize > self.len() {
            return Err("Invalid range");
        } 
        let mut c = self.to_vec().into_iter();
        for _ in 0..start {
            c.next();
        }
        let mut count = 0;
        let mut res: Vec<u8> = vec![];
        let _: Vec<()> = c.map(|i| {
            if count < end {
                res.push(i);
            }
            count += 1;
        }).collect();
        return Ok(res);
        
    }

    fn get_from_offset(&self, start: u8) -> Result<Vec<u8>, &'static str> {
        if start as usize > self.len() {
            return Err("Start index out of bounds.");
        }
        let c = self.to_vec().into_iter();
        let mut count = 0;
        let mut res: Vec<u8> = vec![];
        let _: Vec<()> = c.map(|i| {
            if count >= start {
                res.push(i);
            }
            count += 1;
        }).collect();
        return Ok(res);
    }

    fn to_socket_msg(&self) -> Result<&[u8], &'static str> {
        return Ok(self.as_slice());
    }
}

#[cfg(test)]
mod tests {
    use crate::Blob;
    use crate::Row;

    #[test]
    fn test_slicing() {
        let a: Vec<u8> = vec![1,2,3,4,5];
        let expect: Vec<u8> = vec![3,4,5];
        let res: Vec<u8> = a.get_slice(2, 5).unwrap();
        for (idx, i) in res.iter().enumerate() {
            assert_eq!(&expect[idx], i);
        } 
        assert_eq!(a.len(), 5);
    }

    #[test]
    fn test_get_from_offset() {
        let a: Vec<u8> = vec![1,2,3,4,5];
        let expect: Vec<u8> = vec![3,4,5];
        let res: Vec<u8> = a.get_from_offset(2).unwrap();
        for (idx, i) in res.iter().enumerate() {
            assert_eq!(&expect[idx], i);
        } 
        assert_eq!(a.len(), 5);
    }

    #[test]
    fn test_row_trait() {
        let a: [u8;2] = [0,1];
        let expect_u16: u16 = 1;
        assert_eq!(a.as_u16(), expect_u16);
    }
}
