use crate::df::bit_value::*;
use crate::df::{assembler::Assembler, parser::Parser};
use crate::msg::GpsSigId;
use crate::rtcm_error::RtcmError;
#[cfg(feature = "test_gen")]
use crate::source_repr::SourceRepr;
use crate::util::DataVec;
#[cfg(feature = "serde")]
use crate::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
pub struct Msg1059CodeBias {
    pub sat_id: u8,
    pub sig_id: GpsSigId,
    pub bias_m: f32,
}

#[cfg(feature = "test_gen")]
impl SourceRepr for Msg1059CodeBias {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        write!(
            f,
            "Msg1059CodeBias {{ sat_id: {}, sig_id: gps::",
            self.sat_id
        )?;
        self.sig_id.to_source(f)?;
        write!(f, ", bias_m: ")?;
        self.bias_m.to_source(f)?;
        f.write_char('}')
    }
}

pub mod export_types {
    pub use super::Msg1059CodeBias;
}

pub type DataType = DataVec<Msg1059CodeBias, 390>;

macro_rules! sig_mappings {
    [
        $( $num:literal => $band:literal|$attr:literal ),+
    ] => {
        fn to_sig(id: u8) -> Option<GpsSigId> {
            match id {
                $(
                    $num => Some(GpsSigId::new($band,$attr)),
                )+
                _ => None,
            }
        }
        fn to_id(sig: GpsSigId) -> Option<u8> {
            match (sig.band(), sig.attribute()) {
                $(
                    ($band, $attr) => Some($num),
                )+
                _ => None,
            }
        }
    };
}

sig_mappings![
    0 => 1|'C',
    1 => 1|'P',
    2 => 1|'W',
    5 => 2|'C',
    6 => 2|'D',
    7 => 2|'S',
    8 => 2|'L',
    9 => 2|'X',
    10 => 2|'P',
    11 => 2|'W',
    14 => 5|'I',
    15 => 5|'Q'
];

#[allow(unused)]
pub fn encode(asm: &mut Assembler, value: &DataType) -> Result<(), RtcmError> {
    let mut sat_mask: u64 = 0;
    let mut sat_num: u8 = 0;
    for bias in value.iter() {
        if bias.sat_id <= 63 {
            let sat = 1 << bias.sat_id;
            if sat_mask & sat == 0 {
                sat_mask |= 1 << bias.sat_id;
                sat_num += 1;
            }
        } else {
            return Err(RtcmError::OutOfRange);
        }
    }
    //encode satellite length
    asm.put::<U8>(sat_num, 6)?;

    for s in 0..=63u8 {
        if sat_mask & (1 << s) != 0 {
            asm.put::<U8>(s, 6)?;
            let num_biases: u8 = value
                .iter()
                .filter(|b| b.sat_id == s && to_id(b.sig_id).is_some())
                .count() as u8;
            asm.put::<U8>(num_biases, 5)?;
            let mut bias_mask: u32 = 0;
            for bias in value.iter().filter(|b| b.sat_id == s) {
                if let Some(sig_id) = to_id(bias.sig_id) {
                    asm.put::<U8>(sig_id, 5)?;
                    let mut bias = bias.bias_m;
                    bias /= 0.01;
                    let bias = if bias > 0.0 { bias + 0.5 } else { bias - 0.5 } as i16;
                    asm.put::<I16>(bias, 14)?;
                }
            }
        }
    }
    Ok(())
}

#[allow(unused)]
pub fn decode(par: &mut Parser) -> Result<DataType, RtcmError> {
    let mut value = DataVec::<Msg1059CodeBias, 390>::new();
    let sat_num = par.parse::<U8>(6)?;
    for _ in 0..sat_num {
        let sat_id = par.parse::<U8>(6)?;
        let bias_num = par.parse::<U8>(5)?;
        for _ in 0..bias_num {
            if let Some(sig_id) = to_sig(par.parse::<U8>(5)?) {
                let bias = par.parse::<I16>(14)? as f32;
                value.push(Msg1059CodeBias {
                    sat_id,
                    sig_id,
                    bias_m: bias * 0.01,
                });
            }
        }
    }
    Ok(value)
}

#[cfg(feature = "test_gen")]
use crate::val_gen::ValGen;
#[cfg(feature = "test_gen")]
#[allow(unused)]
pub fn generate<FR, LR, RR>(
    asm: &mut Assembler,
    val_gen: &mut ValGen<FR, LR, RR>,
) -> Result<(), RtcmError>
where
    FR: rand::Rng,
    LR: rand::Rng,
    RR: rand::Rng,
{
    let sat_num = val_gen.len_rng.gen::<u8>() % 64;
    let is_max = val_gen.len_rng.gen::<u64>() == u64::MAX;
    asm.put::<U8>(sat_num, 6)?;
    for s in 1..=sat_num {
        asm.put::<U8>(s, 6)?;
        let num_biases: u8 = if s == 63 { 18 } else { 6 };
        asm.put::<U8>(num_biases, 5)?;
        let mut sig_id = val_gen.rng_rng.gen::<u8>() % 16;
        for _ in 0..num_biases {
            while to_sig(sig_id).is_none() {
                sig_id += 1;
                sig_id %= 16;
            }
            asm.put::<U8>(sig_id, 5)?;
            let bias = val_gen.field_rng.gen::<i16>();
            asm.put::<I16>(bias, 14)?;
            sig_id += 1;
            sig_id %= 16;
        }
    }
    Ok(())
}
