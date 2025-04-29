pub(crate) trait HasBits {
    fn bit_is_set(&self, pos: usize) -> bool;
    fn set_bits(&mut self, map: Self);
    fn unset_bits(&mut self, map: Self);
}

impl HasBits for u8 {
    fn bit_is_set(&self, pos: usize) -> bool {
        for n in 0..8 {
            if n == pos {
                return 1 == self >> n & 1;
            }
        }
        return false;
    }

    fn set_bits(&mut self, map: Self) {
        *self |= map;
    }

    fn unset_bits(&mut self, map: Self) {
        *self &= map;
    }
}

pub(crate) trait Byte {
    fn left_nibble(&self) -> Self;
    fn right_nibble(&self) -> Self;
}

impl Byte for u8 {
    fn left_nibble(&self) -> Self {
        let mut res: u8 = 0;
        let mut dec: u8 = 1;
        for n in 0..4 {
            res += dec * (self >> n & 1);
            dec *= 2;
        }
        return res;
    }

    fn right_nibble(&self) -> Self {
        let mut res: u8 = 0;
        let mut dec: u8 = 1;
        for n in 4..8 {
            res += dec * (self >> n & 1);
            dec *= 2;
        }
        return res;
    }
}
