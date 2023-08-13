use crate::df::bit_value::*;
use crate::rtcm_error::RtcmError;

pub struct Parser<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8], offset: usize) -> Self {
        Parser { data, offset }
    }
    #[allow(unused)]
    pub fn offset(&self) -> usize {
        self.offset
    }
    #[allow(unused)]
    pub fn data(&self) -> &[u8] {
        &self.data[self.offset / 8..]
    }
    pub fn parse<IT: BitValue>(
        &mut self,
        len: usize,
    ) -> Result<<IT as BitValue>::ValueType, RtcmError> {
        if self.data.len() * 8 < self.offset + len {
            Err(RtcmError::BufferOverflow)
        } else {
            let mut val: <IT as BitValue>::ValueType =
                <<IT as BitValue>::ValueType as Default>::default();
            let lh_st = self.offset % 8;
            let lh_en = (self.offset + len) % 8;
            let rh_en = (8 - lh_en) % 8;
            let sti = self.offset / 8;
            let dlen = (self.offset + len - 1) / 8 - sti + 1;
            let mut lenlft = len;
            for (i, d) in self.data.iter().skip(sti).take(dlen).enumerate() {
                let mut b = *d;
                let mut nbits: usize = 8;
                let mut bpos: usize = 0;
                if i == 0 {
                    b &= 255u8 >> lh_st;
                    nbits -= lh_st;
                }
                if i == dlen - 1 {
                    b &= 255u8 << rh_en;
                    nbits -= rh_en;
                    bpos = rh_en;
                }
                lenlft -= nbits;

                if bpos >= lenlft {
                    b >>= bpos - lenlft;
                }
                let bval = <IT as BitValue>::u8_cast(b);

                val |= if bpos >= lenlft {
                    bval
                } else {
                    bval << lenlft - bpos
                };
            }
            self.offset += len;
            Ok(<IT as BitValue>::sign_fix(val, len))
        }
    }
    #[allow(unused)]
    pub fn consume_bits(&mut self, len: usize) {
        self.offset += len;
    }
}

#[cfg(test)]
mod test_parser {
    use super::*;
    #[test]
    fn test_parse() {
        let data = [
            0b11000011u8,
            0b10011010,
            0b11100011,
            0b10101010,
            0b11110000,
            0b11001100,
            0b11111110,
            0b11000011
        ];
        // 1
        let mut parser = Parser::new(&data, 15);
        let ext = parser.parse::<U16>(10).unwrap();
        assert_eq!(0b0111000111, ext);
        // 2
        let mut parser = Parser::new(&data, 13);
        let ext = parser.parse::<U8>(2).unwrap();
        assert_eq!(0b1, ext);
        // 3
        let mut parser = Parser::new(&data, 0);
        let ext = parser.parse::<U16>(12).unwrap();
        assert_eq!(0b110000111001, ext);
        // 4
        let mut parser = Parser::new(&data, 42);
        let ext = parser.parse::<U8>(6).unwrap();
        assert_eq!(0b001100, ext);
        // 5
        let mut parser = Parser::new(&data, 0);
        let ext = parser.parse::<U64>(48).unwrap();
        assert_eq!(0b110000111001101011100011101010101111000011001100, ext);
        // 6
        let mut parser = Parser::new(&data, 4);
        let ext = parser.parse::<U32>(32).unwrap();
        assert_eq!(0b00111001101011100011101010101111, ext);
        // 7
        let mut parser = Parser::new(&data, 18);
        let ext = parser.parse::<I8>(5).unwrap();
        assert_eq!(0b11110001, ext as u8);
        // 8
        let mut parser = Parser::new(&data, 54);
        let ext = parser.parse::<I8>(8).unwrap();
        assert_eq!(0b10110000, ext as u8);        
        // 9
        let mut parser = Parser::new(&data, 12);
        let ext = parser.parse::<SM16>(12).unwrap();
        assert_eq!(-0b1011100011, ext);
    }
}
