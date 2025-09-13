use utility::Blob;

#[derive(Clone)]
pub struct Label {
    length: u8,
    is_compressed: bool,
    offset: u8,
    bytes: Vec<u8>,
}

impl Label {
    pub fn new(length: u8, is_compressed: bool, offset: u8, bytes: Vec<u8>) -> Self {
        return Self {
            length,
            is_compressed,
            offset,
            bytes,
        };
    }

    pub fn from_string(name: String) -> Result<Self, &'static str> {
        let bytes = name.as_bytes().to_vec();
        if bytes.len() > 255 {
            return Err("Name exceeds max size for Label.");
        }
        return Ok(Label {
            length: bytes.len() as u8,
            is_compressed: false,
            offset: 0,
            bytes,
        });
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![self.length];
        res.extend(self.bytes.to_vec());
        return res;
    }

    pub fn read_labels(data: Vec<u8>) -> Result<Vec<Self>, &'static str> {
        let mut labels: Vec<Self> = vec![];
        let mut data_vec = data.to_vec().into_iter();
        let mut complet = false;
        let mut len: u8;
        while !complet {
            if data_vec.len() == 0 {
                complet = true;
                continue;
            }
            len = data_vec.next().unwrap();
            if len == 0 {
                complet = true;
                continue;
            } else if len == 192 {
                let offset = data_vec.next().unwrap();
                labels.push(Label {
                    length: len,
                    is_compressed: true,
                    offset,
                    bytes: vec![192, offset],
                });
                complet = true;
                continue;
            } else {
                let mut b: Vec<u8> = vec![];
                let mut counter: u8 = 0;
                while counter < len {
                    b.push(data_vec.next().unwrap());
                    counter += 1;
                }
                labels.push(Label {
                    length: len,
                    is_compressed: false,
                    offset: 0,
                    bytes: b,
                });
                continue;
            }
        }
        if labels.len() == 0 {
            return Err("Something went wrong parsing the labels.");
        } else {
            return Ok(labels);
        }
    }

    pub fn get_string(&self) -> String {
        if self.is_compressed {
            return String::from("");
        } else {
            return String::from_iter(
                self.bytes
                    .to_vec()
                    .into_iter()
                    .map(|i| i as char)
                    .collect::<Vec<char>>()
                    .into_iter(),
            );
        }
    }

    pub fn decompress(&self, data: Vec<u8>) -> Result<Vec<Label>, &'static str> {
        if self.offset as usize >= data.len() {
            return Err("Cant decompress from out ouf bound offset.");
        }
        if !self.is_compressed {
            return Err("Cant decompress uncompressed labels");
        } else {
            let sliced = data.to_vec().get_from_offset(self.offset).unwrap();
            let labels = Label::read_labels(sliced).unwrap();
            return Ok(labels);
        }
    }

    pub fn is_compressed(&self) -> bool {
        return self.is_compressed;
    }

    pub fn offset(&self) -> u8 {
        self.offset
    }
}

pub struct Name {
    labels: Vec<Label>,
    compressed: bool,
}

impl Clone for Name {
    fn clone(&self) -> Self {
        return Name {
            labels: self.labels.to_vec(),
            compressed: self.compressed.clone(),
        };
    }
}

impl Name {
    pub fn new(labels: Vec<Label>, compressed: bool) -> Self {
        return Self { labels, compressed };
    }

    pub fn from_string(name: String) -> Result<Self, &'static str> {
        if !name.is_ascii() {
            return Err("Names can only contain ascii symbols.");
        }
        let parts = name.split(".");
        let mut labels: Vec<Label> = vec![];
        for part in parts.into_iter() {
            labels.push(Label::from_string(part.to_string()).unwrap());
        }
        return Ok(Name {
            labels,
            compressed: false,
        });
    }

    pub fn get_bytes_length(&self) -> u8 {
        let mut len: u8 = 0;
        if !self.compressed {
            len += 1;
        }
        for i in self.labels.to_vec().into_iter() {
            if i.is_compressed {
                len += 2;
                break;
            }
            len += i.length + 1;
        }
        return len;
    }

    pub fn from_bytes(data: Vec<u8>, offset: u8) -> Self {
        let c = data.to_vec().get_from_offset(offset).unwrap();
        let labels = Label::read_labels(c).unwrap();
        let mut compressed = false;
        for i in labels.to_vec().into_iter() {
            if i.is_compressed {
                compressed = true;
                break;
            }
        }
        return Name { labels, compressed };
    }

    pub fn get_string(&self) -> Result<String, &'static str> {
        if self.compressed {
            return Err("Cant convert compressed Name to String.");
        }
        let mut labels: Vec<String> = vec![];
        for i in self.labels.to_vec().into_iter() {
            labels.push(i.get_string());
        }
        return Ok(labels.join("."));
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        for i in self.labels.to_vec().into_iter() {
            res.extend(i.to_bytes());
        }
        if !self.compressed {
            res.push(0);
        }
        return res;
    }

    pub fn decompress(&self, data: Vec<u8>) -> Result<Self, &'static str> {
        if !self.compressed {
            return Err("Cant decompress uncompressed Name.");
        }

        let mut labels: Vec<Label> = vec![];
        for i in self.labels.to_vec().into_iter() {
            if i.is_compressed {
                let decompressed_labels = i.decompress(data.to_vec()).unwrap();
                labels.extend(decompressed_labels);
                continue;
            } else {
                labels.push(i);
                continue;
            }
        }
        return Ok(Name {
            labels,
            compressed: false,
        });
    }

    pub fn is_compressed(&self) -> bool {
        return self.compressed;
    }
}

#[cfg(test)]
mod tests {
    use crate::name::Label;
    use crate::name::Name;

    #[test]
    fn test_read_labels() {
        let data: Vec<u8> = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
        let res = Label::read_labels(data).unwrap();
        let expected_strings: Vec<String> = vec![String::from("google"), String::from("com")];
        for (idx, v) in res.iter().enumerate() {
            assert_eq!(expected_strings[idx], v.get_string());
        }
    }

    #[test]
    fn test_decompressing() {
        let data: Vec<u8> = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
        let compressed: Vec<u8> = vec![192, 7];
        let res = Label::read_labels(compressed).unwrap();
        assert_eq!(res[0].is_compressed, true);
        let decompressed = res[0].decompress(data).unwrap();
        let expected_strings: Vec<String> = vec![String::from("com")];
        for (idx, v) in decompressed.iter().enumerate() {
            assert_eq!(expected_strings[idx], v.get_string());
        }
    }

    #[test]
    fn test_name_decompression() {
        let data: Vec<u8> = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
        let compressed: Vec<u8> = vec![3, 97, 112, 105, 192, 0];
        let expected_name: String = String::from("api.google.com");
        let compressed_name = Name::from_bytes(compressed, 0);
        let decompressed = compressed_name.decompress(data.to_vec()).unwrap();
        assert_eq!(expected_name, decompressed.get_string().unwrap());
        assert_eq!(16, decompressed.get_bytes_length());
    }

    #[test]
    fn test_name_from_string() {
        let data: Vec<u8> = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
        let name = Name::from_string(String::from("google.com")).unwrap();
        for (idx, v) in name.get_bytes().iter().enumerate() {
            assert_eq!(&data[idx], v);
        }
    }
}
