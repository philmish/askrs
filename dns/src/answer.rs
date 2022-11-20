use crate::name::Name;

pub struct Answer {
    _name: Name,
    _type: [u8;2],
    _class: [u8;2],
    _length: [u8;4],
}
