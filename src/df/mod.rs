
pub mod bit_value;
pub mod assembler;
pub mod parser;

macro_rules! df {
    (
        id: $id:ident,
        dt: $dt:ty,
        it: $it:ty,
        len: $len:literal,
        $(res: $res:expr,)?
        $(round: $round:literal,)?
        $(cap: $cap:expr,$cap_name:ident,)?
        $(inv: $inv:literal,)?
        $(ord: $ord:literal,)?) => {

        pub mod $id {
            use $crate::df::bit_value::*;
            use $crate::df::{assembler::Assembler, parser::Parser};
            
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
            pub fn encode(asm:&mut Assembler, value:&DataType) -> Result<(),()> {
                $(
                    if value.is_none() {
                        return asm.put::<$it>($inv, $len)
                    }
                    let value = &value.unwrap();
                )?
                #[allow(unused_mut)]
                let mut value = *value;
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
            pub fn decode(par:&mut Parser) -> Result<DataType,()> {
                if let Ok(value) = par.parse::<$it>($len) {
                    let dt_val = value as $dt $( * $res )?;
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
                } else {
                    Err(())
                }
            }
            $(
                pub const $cap_name:usize = $cap;
                pub use $cap_name as CAP;
            )?
        }
    };
}

macro_rules! df_88591_string {
    (
        id: $id:ident,
        cap_id: $cap_id:ident,
    ) => {
        pub mod $id {
            use super::*;
            use $crate::util::Df88591String;
            use $crate::df::{assembler::Assembler, parser::Parser, bit_value::U8};

            pub mod export_types {}
            
            pub type DataType = Df88591String<{ $cap_id::CAP }>;
            pub fn encode(asm: &mut Assembler, value: &DataType) -> Result<(), ()> {
                for v in value.iter() {
                    if asm.put::<U8>(*v, 8).is_err() {
                        return Err(());
                    }
                }
                Ok(())
            }
            pub fn decode(par: &mut Parser, len: usize) -> Result<DataType, ()> {
                if len > $cap_id::CAP {
                    return Err(());
                }
                let mut value = Df88591String::new();
                for _ in 0..len {
                    if let Ok(v) = par.parse::<U8>(8) {
                        value.push(v);
                    } else {
                        return Err(());
                    }
                }
                Ok(value)
            }
        }
    };
}

pub mod dfs;
pub use dfs::*;
