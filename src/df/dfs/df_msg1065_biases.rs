use crate::df::bit_value::*;
use crate::df::{assembler::Assembler, parser::Parser};
use crate::msg::GloSigId;
use crate::msg::SAT_CAP_1065;
use crate::rtcm_error::RtcmError;
#[cfg(feature = "test_gen")]
use crate::source_repr::SourceRepr;
use crate::util::DataVec;
#[cfg(feature = "serde")]
use crate::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
pub struct Msg1065CodeBias {
    pub satellite_id: u8,
    pub signal_id: GloSigId,
    pub bias_m: f32,
}

#[cfg(feature = "test_gen")]
impl SourceRepr for Msg1065CodeBias {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        write!(
            f,
            "Msg1065CodeBias {{ satellite_id: {}, signal_id: glo::",
            self.satellite_id
        )?;
        self.signal_id.to_source(f)?;
        write!(f, ", bias_m: ")?;
        self.bias_m.to_source(f)?;
        f.write_char('}')
    }
}

pub mod export_types {
    pub use super::Msg1065CodeBias;
}

pub type DataType = DataVec<Msg1065CodeBias, SAT_CAP_1065>;

macro_rules! sig_mappings {
    [
        $( $num:literal => $band:literal|$attr:literal ),+
    ] => {
        fn to_sig(id: u8) -> Option<GloSigId> {
            match id {
                $(
                    $num => Some(GloSigId::new($band,$attr)),
                )+
                _ => None,
            }
        }
        fn to_id(sig: GloSigId) -> Option<u8> {
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
    2 => 2|'C',
    3 => 2|'P'
];

#[allow(unused)]
pub fn encode(asm: &mut Assembler, value: &DataType) -> Result<(), RtcmError> {
    let mut sat_mask: u32 = 0;
    let mut sat_num: u8 = 0;
    for bias in value.iter() {
        if bias.satellite_id <= 31 {
            let sat = 1 << bias.satellite_id;
            if sat_mask & sat == 0 {
                sat_mask |= 1 << bias.satellite_id;
                sat_num += 1;
            }
        } else {
            return Err(RtcmError::OutOfRange);
        }
    }
    //encode satellite length
    asm.put::<U8>(sat_num, 6)?;

    for s in 0..=31u8 {
        if sat_mask & (1 << s) != 0 {
            asm.put::<U8>(s, 5)?;
            let num_biases: u8 = value
                .iter()
                .filter(|b| b.satellite_id == s && to_id(b.signal_id).is_some())
                .count() as u8;
            asm.put::<U8>(num_biases, 5)?;
            let mut bias_mask: u32 = 0;
            for bias in value.iter().filter(|b| b.satellite_id == s) {
                if let Some(sig_id) = to_id(bias.signal_id) {
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
    let mut value = DataVec::<Msg1065CodeBias, SAT_CAP_1065>::new();
    let sat_num = par.parse::<U8>(6)?;
    for _ in 0..sat_num {
        let satellite_id = par.parse::<U8>(5)?;
        let bias_num = par.parse::<U8>(5)?;
        for _ in 0..bias_num {
            if let Some(signal_id) = to_sig(par.parse::<U8>(5)?) {
                let bias = par.parse::<I16>(14)? as f32;
                value.push(Msg1065CodeBias {
                    satellite_id,
                    signal_id,
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
    let sat_num = val_gen.len_rng.gen::<u8>() % 32;
    let is_max = val_gen.len_rng.gen::<u64>() == u64::MAX;
    asm.put::<U8>(sat_num, 6)?;
    for s in 1..=sat_num {
        asm.put::<U8>(s, 5)?;
        let num_biases: u8 = if s == 31 { 18 } else { 12 };
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
