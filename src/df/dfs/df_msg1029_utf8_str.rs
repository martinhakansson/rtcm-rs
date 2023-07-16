use crate::df::bit_value::*;
use crate::df::{assembler::Assembler, parser::Parser};
use crate::rtcm_error::RtcmError;
use crate::util::ArrayString;

pub mod export_types {}

pub type DataType = ArrayString<255>;

#[allow(unused)]
pub fn encode(asm: &mut Assembler, value: &DataType) -> Result<(), RtcmError> {
    let utf8str = &**value;
    let byte_len = utf8str.bytes().len();
    let char_len = utf8str.chars().count();
    if byte_len > 255 || char_len > 127 {
        return Err(RtcmError::BufferOverflow);
    }

    asm.put::<U8>(char_len as u8, 7)?;
    asm.put::<U8>(byte_len as u8, 8)?;

    for b in utf8str.bytes() {
        asm.put::<U8>(b, 8)?;
    }

    Ok(())
}

#[allow(unused)]
pub fn decode(par: &mut Parser) -> Result<DataType, RtcmError> {
    let _ = par.parse::<U8>(7)?;
    let len = par.parse::<U8>(8)? as usize;
    let data = par.data();
    if data.len() < len {
        return Err(RtcmError::BufferOverflow);
    }
    if let Ok(utf8_str) = core::str::from_utf8(&data[..len]) {
        let array_string = ArrayString::from(utf8_str);
        par.consume_bits(len * 8);
        Ok(array_string)
    } else {
        par.consume_bits(len * 8);
        Err(RtcmError::InvalidUtf8String)
    }
}

#[cfg(feature = "test_gen")]
use crate::val_gen::ValGen;
#[cfg(feature = "test_gen")]
#[allow(unused)]
pub fn generate<FR, LR, RR>(
    asm: &mut Assembler,
    val_gen: &mut ValGen<FR, LR, RR>,
) -> Result<DataType, RtcmError>
where
    FR: rand::Rng,
    LR: rand::Rng,
    RR: rand::Rng,
{
    let mut array_string = ArrayString::<255>::new();
    let char_len = val_gen.len_rng.gen::<usize>() % 128;
    for i in 0..char_len {
        let ch: char = match i {
            i if i <= 25 => char::from_u32(0x61 + i as u32).unwrap(),
            i if i <= 50 => char::from_u32(0x190 + i as u32).unwrap(),
            i if i <= 75 => char::from_u32(0x430 + i as u32).unwrap(),
            i if i <= 100 => char::from_u32(0x680 + i as u32).unwrap(),
            _ => char::from_u32(0x4e00 + i as u32).unwrap(),
        };
        if array_string.try_push(ch).is_err() {
            break;
        }
    }
    let char_len = array_string.chars().count();
    let byte_len = array_string.bytes().len();

    asm.put::<U8>(char_len as u8, 7)?;
    asm.put::<U8>(byte_len as u8, 8)?;

    for b in array_string.bytes() {
        asm.put::<U8>(b, 8)?;
    }

    Ok(array_string)
}
