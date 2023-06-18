use crate::df::bit_value::BitValue;
use crate::rtcm_error::RtcmError;

pub struct Assembler<'a> {
    data: &'a mut [u8],
    offset: usize,
}

impl<'a> Assembler<'a> {
    pub fn new(data: &'a mut [u8], offset: usize) -> Self {
        Assembler { data, offset }
    }
    //#[inline(always)]
    pub fn offset(&self) -> usize {
        self.offset
    }
    pub fn put<IT: BitValue>(
        &mut self,
        value: <IT as BitValue>::ValueType,
        len: usize,
    ) -> Result<(), RtcmError> {
        if self.data.len() * 8 < self.offset + len {
            Err(RtcmError::BufferOverflow)
        } else {
            let value = <IT as BitValue>::sign_fix_rev(value, len);
            let lh_st = self.offset % 8;
            //let rh_st = (8 - lh_st) % 8;
            let lh_en = (self.offset + len) % 8;
            let rh_en = (8 - lh_en) % 8;
            let sti = self.offset / 8;
            let dlen = (self.offset + len - 1) / 8 - sti + 1;
            let mut lenlft = len;
            for (i, d) in self.data.iter_mut().skip(sti).take(dlen).enumerate() {
                let mut bset = 255u8;
                let mut nbits: usize = 8;
                let mut bpos: usize = 0;
                if i == 0 {
                    bset &= 255u8 >> lh_st;
                    nbits -= lh_st;
                }
                if i == dlen - 1 {
                    bset &= 255u8 << rh_en;
                    nbits -= rh_en;
                    bpos = rh_en;
                }
                lenlft -= nbits;

                let tval = if bpos >= lenlft {
                    value << bpos - lenlft
                } else {
                    value >> lenlft - bpos
                };
                let bval = <IT as BitValue>::val_cast(tval);
                //set zeros
                *d &= (!bset) | bval;
                //set ones
                *d |= bset & bval;
            }
            self.offset += len;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test_assembler {
    use super::super::parser::*;
    use super::*;
    use crate::df::bit_value::*;

    #[test]
    fn test_put() {
        let mut data = [
            0b11000011u8,
            0b10011010,
            0b11100011,
            0b10101010,
            0b11110000,
            0b11001100,
        ];
        // 1
        let mut assembler = Assembler::new(&mut data, 15);
        assembler.put::<U16>(0b0101011110, 10).unwrap();
        let mut parser = Parser::new(&data, 15);
        let ext = parser.parse::<U16>(10).unwrap();
        assert_eq!(0b0101011110, ext);
        // 2
        let mut assembler = Assembler::new(&mut data, 13);
        assembler.put::<U8>(0b10, 2).unwrap();
        let mut parser = Parser::new(&data, 13);
        let ext = parser.parse::<U8>(2).unwrap();
        assert_eq!(0b10, ext);
        // 3
        let mut assembler = Assembler::new(&mut data, 0);
        assembler.put::<U16>(0b010101111011, 12).unwrap();
        let mut parser = Parser::new(&data, 0);
        let ext = parser.parse::<U16>(12).unwrap();
        assert_eq!(0b010101111011, ext);
        // 4
        let mut assembler = Assembler::new(&mut data, 42);
        assembler.put::<U8>(0b110011, 6).unwrap();
        let mut parser = Parser::new(&data, 42);
        let ext = parser.parse::<U8>(6).unwrap();
        assert_eq!(0b110011, ext);
        // 5
        let mut assembler = Assembler::new(&mut data, 0);
        assembler
            .put::<U64>(0b110110101001101000100110101000100001000011000101, 48)
            .unwrap();
        let mut parser = Parser::new(&data, 0);
        let ext = parser.parse::<U64>(48).unwrap();
        assert_eq!(0b110110101001101000100110101000100001000011000101, ext);
        // 6
        let mut assembler = Assembler::new(&mut data, 4);
        assembler
            .put::<U32>(0b11001101000000001111101010100011, 32)
            .unwrap();
        let mut parser = Parser::new(&data, 4);
        let ext = parser.parse::<U32>(32).unwrap();
        assert_eq!(0b11001101000000001111101010100011, ext);
        // 6
        let mut assembler = Assembler::new(&mut data, 18);
        assembler.put::<I8>(-12, 5).unwrap();
        let mut parser = Parser::new(&data, 18);
        let ext = parser.parse::<I8>(5).unwrap();
        assert_eq!(-12, ext);
        // 6
        let mut assembler = Assembler::new(&mut data, 12);
        assembler.put::<SM16>(-1820, 12).unwrap();
        let mut parser = Parser::new(&data, 12);
        let ext = parser.parse::<SM16>(12).unwrap();
        assert_eq!(-1820, ext);
    }
}
