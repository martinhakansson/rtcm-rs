pub mod assembler;
pub mod bit_value;
pub mod parser;

macro_rules! df {
    (
        id: $id:ident,
        dt: $dt:ty,
        it: $it:ty,
        len: $len:literal,
        $(res: $res:expr,)?
        $(bias: $bias:expr,)?
        $(round: $round:literal,)?
        $(cap: $cap:expr,$cap_name:ident,)?
        $(inv: $inv:literal,)?
        $(ord: $ord:literal,)?) => {

        pub mod $id {
            use $crate::df::bit_value::*;
            use $crate::df::{assembler::Assembler, parser::Parser};
            use $crate::rtcm_error::RtcmError;

            pub mod export_types {
                $(pub use super::$cap_name;)?
            }

            $(
                pub type DataType = $dt;
                const _:<$it as BitValue>::ValueType = $ord;
            )?
            $(
                pub type DataType = Option<$dt>;
                const _:<$it as BitValue>::ValueType = $inv;
            )?
            #[allow(unused)]
            pub fn encode(asm:&mut Assembler, value:&DataType) -> Result<(),RtcmError> {
                $(
                    if value.is_none() {
                        return asm.put::<$it>($inv, $len)
                    }
                    let value = &value.unwrap();
                )?
                #[allow(unused_mut)]
                let mut value = *value;
                $(
                    if value >= $bias {
                        value -= $bias;
                    } else {
                        return Err(RtcmError::OutOfRange);
                    }
                )?
                $(
                    value /= $res;
                )?
                $(
                    if $round {
                        value += if value >= 0.0 {
                            0.5
                        } else {
                            -0.5
                        };
                    }
                )?
                let value = value as <$it as BitValue>::ValueType;

                asm.put::<$it>(value, $len)
            }
            #[allow(unused)]
            pub fn decode(par:&mut Parser) -> Result<DataType,RtcmError> {

                let value = par.parse::<$it>($len)?;
                #[allow(unused_mut)]
                let mut dt_val = value as $dt $( * $res )?;
                $(
                    dt_val += $bias;
                )?
                $(
                    if value == $inv {
                        return Ok(None);
                    } else {
                        return Ok(Some(dt_val));
                    }
                )?
                $(
                    let _ = $ord;
                    return Ok(dt_val);
                )?

                // if let Ok(value) = par.parse::<$it>($len) {
                //     let dt_val = value as $dt $( * $res )?;
                //     $(
                //         if value == $inv {
                //             return Ok(None);
                //         } else {
                //             return Ok(Some(dt_val));
                //         }
                //     )?
                //     $(
                //         let _ = $ord;
                //         return Ok(dt_val);
                //     )?
                // } else {
                //     Err(())
                // }
            }
            #[cfg(feature = "test_gen")]
            use $crate::val_gen::ValGen;
            #[cfg(feature = "test_gen")]
            #[allow(unused)]
            pub fn generate<FR,LR,RR>(asm:&mut Assembler, val_gen:&mut ValGen<FR,LR,RR>) -> Result<DataType,RtcmError>
                where FR:rand::Rng, LR:rand::Rng, RR:rand::Rng {
                /*
                let it_val = {
                    if rng.gen::<f32>() < 0.2f32 {
                        0u64 as <$it as BitValue>::ValueType
                    } else {
                        if rng.gen::<f32>() < 0.25f32 {
                            0xffffffffffffffff_u64 as <$it as BitValue>::ValueType
                        } else {
                            rng.gen()
                        }
                    }
                }; */
                #[allow(unused)]
                let it_val:<$it as BitValue>::ValueType = if val_gen.field_rng.gen::<u64>() == u64::MAX {
                    $(
                        let _ = $ord;
                        val_gen.field_rng.gen::<<$it as BitValue>::ValueType>()
                    )?
                    $(
                        $inv
                    )?
                } else {
                    val_gen.field_rng.gen::<<$it as BitValue>::ValueType>()
                };
                $(
                    let it_val = val_gen.len_rng.gen::<<$it as BitValue>::ValueType>() % ($cap + 1);
                )?
                asm.put::<$it>(it_val, $len)?;
                let shift = <$it as BitValue>::ValueType::BITS-$len;
                let dt_val = ((it_val << shift) >> shift) as $dt $( * $res )?;
                    $(
                        if it_val == $inv {
                            return Ok(None);
                        } else {
                            return Ok(Some(dt_val));
                        }
                    )?
                    $(
                        let _ = $ord;
                        return Ok(dt_val);
                    )?
            }
            $(
                #[allow(unused)]
                pub const $cap_name:usize = $cap;
                pub use $cap_name as CAP;
            )?
        }
    };
}

macro_rules! df_88591_string {
    (
        id: $id:ident,
        cap_name: $cap_name:ident,
    ) => {
        pub mod $id {
            //use super::*;
            use $crate::df::{assembler::Assembler, bit_value::U8, parser::Parser};
            use $crate::msg::$cap_name;
            use $crate::rtcm_error::RtcmError;
            use $crate::util::Df88591String;

            pub mod export_types {}
            #[allow(unused)]
            pub type DataType = Df88591String<$cap_name>;
            #[allow(unused)]
            pub fn encode(asm: &mut Assembler, value: &DataType) -> Result<(), RtcmError> {
                for v in value.iter() {
                    asm.put::<U8>(*v, 8)?;
                    // if asm.put::<U8>(*v, 8).is_err() {
                    //     return Err(());
                    // }
                }
                Ok(())
            }
            #[allow(unused)]
            pub fn decode(par: &mut Parser, len: usize) -> Result<DataType, RtcmError> {
                if len > $cap_name {
                    return Err(RtcmError::CapacityExceeded);
                }
                let mut value = Df88591String::new();
                for _ in 0..len {
                    let v = par.parse::<U8>(8)?;
                    value.push(v);
                    // if let Ok(v) = par.parse::<U8>(8) {
                    //     value.push(v);
                    // } else {
                    //     return Err(());
                    // }
                }
                Ok(value)
            }
            #[cfg(feature = "test_gen")]
            use $crate::val_gen::ValGen;
            #[cfg(feature = "test_gen")]
            #[allow(unused)]
            pub fn generate<FR, LR, RR>(
                asm: &mut Assembler,
                val_gen: &mut ValGen<FR, LR, RR>,
                len: usize,
            ) -> Result<(), RtcmError>
            where
                FR: rand::Rng,
                LR: rand::Rng,
                RR: rand::Rng,
            {
                let mut value: DataType = Df88591String::new();
                for _ in 0..len {
                    let v = 48 + (val_gen.field_rng.gen::<u8>() % 42);
                    value.push(v);
                }
                for v in value.iter() {
                    asm.put::<U8>(*v, 8)?;
                    // if asm.put::<U8>(*v, 8).is_err() {
                    //     return Err(());
                    // }
                }
                Ok(())
            }
        }
    };
}

pub mod dfs;
pub use dfs::*;
