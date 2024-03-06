use crate::df::bit_value::*;
use crate::df::{assembler::Assembler, parser::Parser};
use crate::msg::GloSigId;
use crate::rtcm_error::RtcmError;
#[cfg(feature = "test_gen")]
use crate::source_repr::SourceRepr;
use crate::util::DataVec;
#[cfg(feature = "serde")]
use crate::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
pub struct Msg1230CodePhaseBias {
    pub signal_id: GloSigId,
    pub bias_m: f32,
}

#[cfg(feature = "test_gen")]
impl SourceRepr for Msg1230CodePhaseBias {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        write!(f, "Msg1230CodePhaseBias {{ signal_id: glo::")?;
        self.signal_id.to_source(f)?;
        write!(f, ", bias_m: ")?;
        self.bias_m.to_source(f)?;
        f.write_char('}')
    }
}

pub mod export_types {
    pub use super::Msg1230CodePhaseBias;
}

pub type DataType = DataVec<Msg1230CodePhaseBias, 4>;

#[allow(unused)]
pub fn encode(asm: &mut Assembler, value: &DataType) -> Result<(), RtcmError> {
    let mut sig_mask: u8 = 0;
    let mut value = value.clone();
    let slice = value.as_mut_slice();
    slice.sort_unstable_by(|a, b| a.signal_id.cmp(&b.signal_id));
    for v in slice.iter() {
        let sig_id = v.signal_id;
        match (sig_id.band(), sig_id.attribute()) {
            (1, 'C') => {
                sig_mask |= 1 << 3;
            }
            (1, 'P') => {
                sig_mask |= 1 << 2;
            }
            (2, 'C') => {
                sig_mask |= 1 << 1;
            }
            (2, 'P') => {
                sig_mask |= 1;
            }
            _ => {
                return Err(RtcmError::InvalidSignalId);
            }
        }
    }
    asm.put::<U8>(sig_mask, 4)?;
    for v in slice.iter() {
        let mut bias = v.bias_m;
        bias /= 0.02;
        let bias = if bias > 0.0 { bias + 0.5 } else { bias - 0.5 } as i16;
        asm.put::<I16>(bias, 16)?;
    }
    Ok(())
}

#[allow(unused)]
pub fn decode(par: &mut Parser) -> Result<DataType, RtcmError> {
    let mut value = DataVec::<Msg1230CodePhaseBias, 4>::new();
    let sig_mask: u8 = par.parse::<U8>(4)?;
    for i in 0..4usize {
        if sig_mask & (1 << (3 - i)) != 0 {
            let bias_m = (par.parse::<I16>(16)? as f32) * 0.02;

            match i {
                0 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(1, 'C'),
                        bias_m,
                    });
                }
                1 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(1, 'P'),
                        bias_m,
                    });
                }
                2 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(2, 'C'),
                        bias_m,
                    });
                }
                3 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(2, 'P'),
                        bias_m,
                    });
                }
                _ => unreachable!(),
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
) -> Result<DataType, RtcmError>
where
    FR: rand::Rng,
    LR: rand::Rng,
    RR: rand::Rng,
{
    let mut value = DataVec::<Msg1230CodePhaseBias, 4>::new();
    let sig_mask: u8 = val_gen.len_rng.gen();
    asm.put::<U8>(sig_mask, 4)?;
    for i in 0..4usize {
        if sig_mask & (1 << (3 - i)) != 0 {
            let raw_val: i16 = val_gen.field_rng.gen();
            let bias_m = (raw_val as f32) * 0.02;

            match i {
                0 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(1, 'C'),
                        bias_m,
                    });
                }
                1 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(1, 'P'),
                        bias_m,
                    });
                }
                2 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(2, 'C'),
                        bias_m,
                    });
                }
                3 => {
                    value.push(Msg1230CodePhaseBias {
                        signal_id: GloSigId::new(2, 'P'),
                        bias_m,
                    });
                }
                _ => unreachable!(),
            }

            asm.put::<I16>(raw_val, 16)?;
        }
    }
    Ok(value)
}
