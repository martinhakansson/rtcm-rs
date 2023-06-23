use tinyvec::ArrayVec;

//use crate::df::{assembler::Assembler, parser::Parser};

pub mod message;

mod msm_mappings;

pub use msm_mappings::bds::SigId as BdsSigId;
pub use msm_mappings::gal::SigId as GalSigId;
pub use msm_mappings::glo::SigId as GloSigId;
pub use msm_mappings::gps::SigId as GpsSigId;
pub use msm_mappings::sbas::SigId as SbasSigId;
pub use msm_mappings::qzss::SigId as QzssSigId;

macro_rules! msg {
    (
        id: $id:ident,
        type_name: $type_name:ident,
        fields: [ $( ( $field_name:ident, $frag_id:ident $(, $len_data:tt )? ) ),+ ],
    ) => {
        pub mod $id {
            use $crate::df::{assembler::Assembler, parser::Parser};
            use $crate::df::dfs::*;
            use $crate::rtcm_error::RtcmError;
            #[cfg(feature = "serde")]
            use $crate::{Serialize,Deserialize};
            #[cfg(feature = "test_gen")]
            use $crate::source_repr::SourceRepr;
            #[allow(unused)]
            use super::*; //Do not remove

            pub mod export_types {
                $(pub use super::$frag_id::export_types::*;)+

                pub use super::$type_name;
            }

            #[derive(Default, Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde",derive(Serialize,Deserialize),serde(crate = "sd"))]
            pub struct $type_name {
                $(pub $field_name:$frag_id::DataType),+
            }
            pub type DataType = $type_name;
            pub fn encode(asm:&mut Assembler, value:&$type_name) -> Result<(),RtcmError> {
                $(
                    $frag_id::encode(asm, &value.$field_name)?;
                    // if $frag_id::encode(asm, &value.$field_name).is_err() {
                    //     return Err(());
                    // }
                )+
                Ok(())
            }
            pub fn decode(par:&mut Parser) -> Result<$type_name,RtcmError> {
                $(
                    #[allow(unused)]
                    let $field_name = $frag_id::decode(par, $($len_data)? )?;
                    // #[allow(unused)]
                    // let $field_name = if let Ok(value) = $frag_id::decode(par, $($len_data)? ) {
                    //     value
                    // } else {
                    //     return Err(());
                    // };
                )+

                Ok($type_name {
                    $(
                        $field_name
                    ),+
                })
            }
            #[cfg(feature = "test_gen")]
            use $crate::val_gen::ValGen;
            #[cfg(feature = "test_gen")]
            pub fn generate<FR,LR,RR>(asm:&mut Assembler, val_gen:&mut ValGen<FR,LR,RR>) -> Result<(),RtcmError>
                where FR:rand::Rng, LR:rand::Rng, RR:rand::Rng {

                $(
                    #[allow(unused)]
                    let $field_name = $frag_id::generate(asm, val_gen, $($len_data)? )?;
                    #[allow(unused)]
                    // let $field_name = if let Ok(value) = $frag_id::generate(asm, val_gen, $($len_data)? ) {
                    //     value
                    // } else {
                    //     return Err(());
                    // };
                )+
                Ok(())
            }
            #[cfg(feature = "test_gen")]
            impl SourceRepr for $type_name {
                fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    use core::fmt::Write;
                    write!(f, "{} {{", stringify!($type_name))?;
                    $(
                        write!(f,"{}:", stringify!($field_name))?;
                        self.$field_name.to_source(f)?;
                        f.write_char(',')?;
                    )+
                    f.write_char('}')
                }
            }
        }
    };
}
#[allow(unused)]
macro_rules! frag_vec {
    (
        id: $id:ident,
        frag_id: $frag_id:ident,
        cap_name: $cap_name:ident,
    ) => {
        pub mod $id {
            use super::*;
            //use $crate::df::dfs::*;
            use $crate::df::{assembler::Assembler, parser::Parser};
            use $crate::msg::*;
            use $crate::util::DataVec;
            use $crate::rtcm_error::RtcmError;
            //pub type DataType = ArrayVec<[$frag_id::DataType; $cap_id::CAP]>;

            pub mod export_types {
                pub use super::$frag_id::export_types::*;
            }

            pub type DataType = DataVec<$frag_id::DataType, $cap_name>;
            pub fn encode(asm: &mut Assembler, value: &DataType) -> Result<(), RtcmError> {
                for v in value.iter() {
                    $frag_id::encode(asm, v)?;
                    // if $frag_id::encode(asm, v).is_err() {
                    //     return Err(());
                    // }
                }
                Ok(())
            }
            pub fn decode(par: &mut Parser, len: usize) -> Result<DataType, RtcmError> {
                if len > $cap_name {
                    return Err(RtcmError::CapacityExceeded);
                }
                let mut value = DataVec::new();
                for _ in 0..len {
                    let v = $frag_id::decode(par)?;
                    // let v = if let Ok(v) = $frag_id::decode(par) {
                    //     v
                    // } else {
                    //     return Err(());
                    // };
                    value.push(v);
                }
                Ok(value)
            }
            #[cfg(feature = "test_gen")]
            use $crate::val_gen::ValGen;
            #[cfg(feature = "test_gen")]
            pub fn generate<FR,LR,RR>(
                asm:&mut Assembler, 
                val_gen:&mut ValGen<FR,LR,RR>,
                len: usize,
            ) -> Result<(), RtcmError>
            where FR:rand::Rng, LR:rand::Rng, RR:rand::Rng {
                for _ in 0..len {
                    $frag_id::generate(asm, val_gen)?;
                    // if $frag_id::generate(asm, val_gen).is_err() {
                    //     return Err(());
                    // }
                }
                Ok(())
            }
        }
    };
}
#[allow(unused)]
macro_rules! msm_data_seg_frag {
    (
        id: $id:ident,
        type_name: $type_name:ident,
        gnss: $gnss:ident,
        sat_id: $sat_id:ident,
        sig_id: $sig_id:ident,
    ) => {
        pub mod $id {
            use super::*;
            use $crate::df::bit_value::{U32, U64};
            use $crate::df::{assembler::Assembler, parser::Parser};
            use $crate::rtcm_error::RtcmError;
            use $crate::msg::{
                cell_mask_id_vec, mask_len_u32, mask_len_u64, msm_mappings::$gnss::*,
            };
            use $crate::tinyvec::ArrayVec;
            #[cfg(feature = "serde")]
            use $crate::{Deserialize, Serialize};

            pub mod export_types {
                pub use super::$sat_id::export_types::*;
                pub use super::$sig_id::export_types::*;

                pub use super::$type_name;
            }

            #[derive(Default, Clone, Debug, PartialEq)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
            pub struct $type_name {
                pub sat_data: $sat_id::DataType,
                pub sig_data: $sig_id::DataType,
            }
            pub type DataType = $type_name;
            pub fn encode(asm: &mut Assembler, value: &$type_name) -> Result<(), RtcmError> {
                let mut sat_mask: u64 = 0;
                for s in value.sat_data.iter() {
                    if s.sat_id > 0 && s.sat_id <= 64 {
                        let sat: u64 = 1 << (64 - s.sat_id);
                        if sat & sat_mask > 0 {
                            return Err(RtcmError::DuplicateSatellite);
                        }
                        sat_mask |= sat;
                    } else {
                        return Err(RtcmError::InvalidSatelliteId);
                    }
                }
                let mut sig_mask: u32 = 0;
                let mut sat_sig_mask: u64 = 0;
                let mut cell_vec: ArrayVec<[(u8, u8); 64]> = ArrayVec::new();
                for s in value.sig_data.iter() {
                    let sat_id = if s.sat_id > 0 && s.sat_id <= 64 {
                        s.sat_id
                    } else {
                        return Err(RtcmError::InvalidSatelliteId);
                    };
                    let sig_id = if let Some(sig_id) = to_id(s.sig_id) {
                        sig_id
                    } else {
                        return Err(RtcmError::InvalidSignalId);
                    };
                    let sat: u64 = 1 << (64 - sat_id);
                    sat_sig_mask |= sat;
                    let sig = 1 << (32 - sig_id);
                    sig_mask |= sig;
                    cell_vec.push((sat_id, sig_id));
                }
                if sat_mask != sat_sig_mask {
                    return Err(RtcmError::SatelliteMismatch);
                }
                let mut sat_indx = [0usize; 64];
                let mut sig_indx = [0usize; 32];
                let mut indx_counter = 0;
                for i in 0..64 {
                    if (sat_mask >> (63 - i)) % 2 == 1 {
                        sat_indx[i] = indx_counter;
                        indx_counter += 1;
                    }
                }
                indx_counter = 0;
                for i in 0..32 {
                    if (sig_mask >> (31 - i)) % 2 == 1 {
                        sig_indx[i] = indx_counter;
                        indx_counter += 1;
                    }
                }
                let sig_mask_len = mask_len_u32(sig_mask);
                let cell_cont_len = sig_mask_len * value.sat_data.len();
                let mut cell_mask: u64 = 0;
                for (sat_id, sig_id) in cell_vec {
                    let cell_indx = sat_indx[sat_id as usize - 1] * sig_mask_len
                        + sig_indx[sig_id as usize - 1];
                    let cell = 1 << (cell_cont_len - 1 - cell_indx);
                    if cell & cell_mask > 0 {
                        return Err(RtcmError::DuplicateSatelliteSignal);
                    }
                    cell_mask |= cell;
                }
                asm.put::<U64>(sat_mask, 64)?;
                // if asm.put::<U64>(sat_mask, 64).is_err() {
                //     return Err(());
                // }
                asm.put::<U32>(sig_mask, 32)?;
                // if asm.put::<U32>(sig_mask, 32).is_err() {
                //     return Err(());
                // }
                asm.put::<U64>(cell_mask, cell_cont_len)?;
                // if asm.put::<U64>(cell_mask, cell_cont_len).is_err() {
                //     return Err(());
                // }
                $sat_id::encode(asm, &value.sat_data)?;
                // if $sat_id::encode(asm, &value.sat_data).is_err() {
                //     return Err(());
                // }
                $sig_id::encode(asm, &value.sig_data)?;
                // if $sig_id::encode(asm, &value.sig_data).is_err() {
                //     return Err(());
                // }
                Ok(())
            }
            pub fn decode(par: &mut Parser) -> Result<$type_name, RtcmError> {
                let sat_mask = par.parse::<U64>(64)?;
                // let sat_mask = if let Ok(v) = par.parse::<U64>(64) {
                //     v
                // } else {
                //     return Err(());
                // };
                let sig_mask = par.parse::<U32>(32)?;
                // let sig_mask = if let Ok(v) = par.parse::<U32>(32) {
                //     v
                // } else {
                //     return Err(());
                // };
                let sat_len = mask_len_u64(sat_mask);
                let sig_len = mask_len_u32(sig_mask);
                let cell_mask = par.parse::<U64>(sat_len * sig_len)?;
                // let cell_mask = if let Ok(v) = par.parse::<U64>(sat_len * sig_len) {
                //     v
                // } else {
                //     return Err(());
                // };
                if let Some((sat_vec, cell_vec)) = cell_mask_id_vec(sat_mask, sig_mask, cell_mask) {
                    let sat_data = $sat_id::decode(par, &sat_vec)?;
                    // let sat_data = if let Ok(v) = $sat_id::decode(par, &sat_vec) {
                    //     v
                    // } else {
                    //     return Err(());
                    // };
                    let sig_data = $sig_id::decode(par, &cell_vec)?;
                    // let sig_data = if let Ok(v) = $sig_id::decode(par, &cell_vec) {
                    //     v
                    // } else {
                    //     return Err(());
                    // };
                    Ok($type_name { sat_data, sig_data })
                } else {
                    return Err(RtcmError::InvalidSatelliteSignalCount);
                }
            }
            #[cfg(feature = "test_gen")]
            use $crate::val_gen::ValGen;
            #[cfg(feature = "test_gen")]
            pub fn generate<FR,LR,RR>(asm:&mut Assembler, val_gen:&mut ValGen<FR,LR,RR>) -> Result<(), RtcmError>
            where FR:rand::Rng, LR:rand::Rng, RR:rand::Rng {
                let sig_len: usize = (val_gen.len_rng.gen::<usize>() % 64) + 1;
                let subset: usize = val_gen.rng_rng.gen::<usize>();
                let mut cell_vec: ArrayVec<[(u8, u8); 64]> = ArrayVec::new();
                let mut sat_mask: u64 = 0;
                let mut sig_mask: u32 = 0;
                let mut sat_num: usize = 0;
                //let mut new_sat_num:usize = 0;
                let mut sig_num: usize = 0;
                //let mut new_sig_num:usize = 0;
                for _ in 0..sig_len {
                    //let mut sat_id:u8 = (rng.gen::<u8>() % 64) + 1;
                    //let mut sig_id = random_id(rng);
                    //let slice = &cell_vec[..];
                    /*
                    while cell_vec.iter().any(|e| e.0 == sat_id && e.1 == sig_id) || new_sat_num * new_sig_num > 64 {
                        new_sat_num = sat_num;
                        new_sig_num = sig_num;
                        sat_id = (rng.gen::<u8>() % 64) + 1;
                        sig_id = random_id(rng);
                        if (sat_mask & (1 << (64 - sat_id))) == 0 {
                            new_sat_num = sat_num + 1;
                        }
                        if (sig_mask & (1 << (32 - sig_id))) == 0 {
                            new_sig_num = sig_num + 1;
                        }
                    }
                     */
                    //let (sat_id, sig_id) = (i as u8 +1, 2u8);
                    
                    let (sat_id, sig_id) = loop {
                        let sat_id: u8 = (((subset % 65) as u8 + (val_gen.rng_rng.gen::<u8>() % 16)) % 64) + 1;
                        let sig_id = random_id(&mut val_gen.rng_rng, subset);
                        /*let new_sat_num = if (sat_mask & (1 << (64 - sat_id))) == 0 {
                            sat_num + 1
                        } else {
                            sat_num
                        };
                        let new_sig_num = if (sig_mask & (1 << (32 - sig_id))) == 0 {
                            sig_num + 1
                        } else {
                            sig_num
                        };*/
                        if !cell_vec.iter().any(|e| e.0 == sat_id && e.1 == sig_id)
                            //&& new_sat_num * new_sig_num <= 64
                        {
                            break (sat_id, sig_id);
                        }
                    };
                    cell_vec.push((sat_id, sig_id));
                    let sat: u64 = 1 << (64 - sat_id);
                    if (sat_mask & sat) == 0 {
                        sat_num += 1;
                    }
                    sat_mask |= sat;
                    let sig: u32 = 1 << (32 - sig_id);
                    if (sig_mask & sig) == 0 {
                        sig_num += 1;
                    }
                    sig_mask |= sig;
                }
                let cell_cont_len = sig_num * sat_num;
                let mut sat_indx = [0usize; 64];
                let mut sig_indx = [0usize; 32];
                let mut indx_counter = 0;
                for i in 0..64 {
                    if (sat_mask >> (63 - i)) % 2 == 1 {
                        sat_indx[i] = indx_counter;
                        indx_counter += 1;
                    }
                }
                indx_counter = 0;
                for i in 0..32 {
                    if (sig_mask >> (31 - i)) % 2 == 1 {
                        sig_indx[i] = indx_counter;
                        indx_counter += 1;
                    }
                }
                let mut cell_mask: u64 = 0;
                for (sat_id, sig_id) in cell_vec {
                    let cell_indx =
                        sat_indx[sat_id as usize - 1] * sig_num + sig_indx[sig_id as usize - 1];
                    let cell: u64 = 1 << (cell_cont_len - 1 - cell_indx);
                    debug_assert_eq!(cell & cell_mask, 0);
                    // if cell & cell_mask > 0 {
                    //     unreachable!();
                    // }
                    cell_mask |= cell;
                }
                asm.put::<U64>(sat_mask, 64)?;
                // if asm.put::<U64>(sat_mask, 64).is_err() {
                //     return Err(());
                // }
                asm.put::<U32>(sig_mask, 32)?;
                // if asm.put::<U32>(sig_mask, 32).is_err() {
                //     return Err(());
                // }
                asm.put::<U64>(cell_mask, cell_cont_len)?;
                // if asm.put::<U64>(cell_mask, cell_cont_len).is_err() {
                //     return Err(());
                // }
                $sat_id::generate(asm, val_gen, sat_mask)?;
                // if $sat_id::generate(asm, val_gen, sat_mask).is_err() {
                //     return Err(());
                // }
                $sig_id::generate(asm, val_gen, sig_len)?;
                // if $sig_id::generate(asm, val_gen, sig_len).is_err() {
                //     return Err(());
                // }

                Ok(())
            }

            #[cfg(feature = "test_gen")]
            impl $crate::source_repr::SourceRepr for $type_name {
                fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    use core::fmt::Write;

                    write!(f, "{} {{", stringify!($type_name))?;
                    f.write_str("sat_data: ")?;
                    self.sat_data.to_source(f)?;
                    f.write_str(", sig_data: ")?;
                    self.sig_data.to_source(f)?;
                    f.write_char('}')?;

                    Ok(())
                }
            }
        }
    };
}
#[allow(unused)]
fn mask_len_u32(sat_mask: u32) -> usize {
    let mut counter: usize = 0;
    for sh in 0..32 {
        counter += ((sat_mask >> sh) % 2) as usize;
    }
    counter
}
#[allow(unused)]
fn mask_len_u64(sat_mask: u64) -> usize {
    let mut counter: usize = 0;
    for sh in 0..64 {
        counter += ((sat_mask >> sh) % 2) as usize;
    }
    counter
}
#[allow(unused)]
macro_rules! msm_sat_frag {
    (
        id: $id:ident,
        type_name: $type_name:ident,
        fields: [ $( ( $field_name:ident, $frag_id:ident ) ),+ ],
    ) => {
        pub mod $id {
            use $crate::util::DataVec;
            use $crate::df::{assembler::Assembler, parser::Parser};
            use $crate::df::dfs::*;
            use $crate::rtcm_error::RtcmError;
            //use super::*;
            use $crate::tinyvec::ArrayVec;
            #[cfg(feature = "serde")]
            use $crate::{Serialize,Deserialize};

            pub mod export_types {
                $(pub use super::$frag_id::export_types::*;)+

                pub use super::$type_name;
            }

            #[derive(Default,Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde",derive(Serialize,Deserialize),serde(crate = "sd"))]
            pub struct $type_name {
                pub sat_id: u8,
                $(pub $field_name:$frag_id::DataType),+
            }
            pub type DataType = DataVec<$type_name,64>;
            pub fn encode(asm:&mut Assembler, value:&DataType) -> Result<(),RtcmError> {
                let mut value = value.clone();
                let slice = value.as_mut_slice();
                slice.sort_unstable_by(|a,b| a.sat_id.cmp(&b.sat_id));
                $(
                    for v in value.iter() {
                        $frag_id::encode(asm, &v.$field_name)?;
                        // if $frag_id::encode(asm, &v.$field_name).is_err() {
                        //     return Err(());
                        // }
                    }
                )+
                Ok(())
            }
            pub fn decode(par:&mut Parser, sat_vec:&ArrayVec<[u8;64]>) -> Result<DataType,RtcmError> {
                let mut value = DataVec::<$type_name,64>::new();
                value.set_len(sat_vec.len());
                {
                    let mut iter = value.iter_mut();
                    for s in sat_vec {
                        let v = iter.next().unwrap();
                        v.sat_id = *s;
                    }
                }
                $(
                    for v in value.iter_mut() {
                        v.$field_name = $frag_id::decode(par)?;
                        // v.$field_name = if let Ok(v) = $frag_id::decode(par) {
                        //     v
                        // } else {
                        //     return Err(());
                        // };
                    }
                )+
                Ok(value)
            }
            #[cfg(feature = "test_gen")]
            use $crate::val_gen::ValGen;
            #[cfg(feature = "test_gen")]
            pub fn generate<FR,LR,RR>(asm:&mut Assembler, val_gen:&mut ValGen<FR,LR,RR>, sat_mask:u64) -> Result<(),RtcmError>
            where FR:rand::Rng, LR:rand::Rng, RR:rand::Rng {

                let mut sat_len:usize = 0;
                for i in 0..64 {
                    if (sat_mask >> (63-i)) % 2 == 1 {
                        sat_len += 1;
                    }
                }
                $(
                    for _ in 0..sat_len {
                        $frag_id::generate(asm, val_gen)?;
                        // if $frag_id::generate(asm, val_gen).is_err() {
                        //     return Err(());
                        // }
                    }
                )+

                Ok(())
            }
            #[cfg(feature = "test_gen")]
            impl $crate::source_repr::SourceRepr for $type_name {
                fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    use core::fmt::Write;

                    write!(f, "{} {{", stringify!($type_name))?;
                    f.write_str("sat_id:")?;
                    self.sat_id.to_source(f)?;
                    f.write_char(',')?;

                    $(
                        write!(f, "{}:", stringify!($field_name))?;
                        self.$field_name.to_source(f)?;
                        f.write_char(',')?;
                    )+
                    f.write_char('}')?;

                    Ok(())
                }
            }
        }
    };
}
#[allow(unused)]
macro_rules! msm_sig_frag {
    (
        id: $id:ident,
        type_name: $type_name:ident,
        gnss: $gnss:ident,
        fields: [ $( ( $field_name:ident, $frag_id:ident ) ),+ ],
    ) => {
        pub mod $id {
            use $crate::util::DataVec;
            use $crate::df::{assembler::Assembler, parser::Parser};
            use $crate::df::dfs::*;
            use $crate::rtcm_error::RtcmError;
            //use super::*;
            use $crate::tinyvec::ArrayVec;
            use $crate::msg::msm_mappings::$gnss::*;
            #[cfg(feature = "serde")]
            use $crate::{Serialize,Deserialize};

            pub mod export_types {
                $(pub use super::$frag_id::export_types::*;)+

                pub use super::$type_name;
            }

            #[derive(Default, Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde",derive(Serialize,Deserialize),serde(crate = "sd"))]
            pub struct $type_name {
                pub sat_id:u8,
                pub sig_id:SigId,
                $(pub $field_name:$frag_id::DataType),+
            }
            pub type DataType = DataVec<$type_name,64>;
            pub fn encode(asm:&mut Assembler, value:&DataType) -> Result<(),RtcmError> {
                let mut value = value.clone();
                let slice = value.as_mut_slice();
                slice.sort_unstable_by(|a,b| {
                    match a.sat_id.cmp(&b.sat_id) {
                        core::cmp::Ordering::Less => core::cmp::Ordering::Less,
                        core::cmp::Ordering::Equal => a.sig_id.cmp(&b.sig_id),
                        core::cmp::Ordering::Greater => core::cmp::Ordering::Greater,
                    }
                });
                $(
                    for v in value.iter() {
                        $frag_id::encode(asm, &v.$field_name)?;
                        // if $frag_id::encode(asm, &v.$field_name).is_err() {
                        //     return Err(());
                        // }
                    }
                )+
                Ok(())
            }
            pub fn decode(par:&mut Parser, cell_vec:&ArrayVec<[(u8,u8);64]>) -> Result<DataType,RtcmError> {
                let mut value = DataVec::<$type_name,64>::new();
                value.set_len(cell_vec.len());
                {
                    let mut iter = value.iter_mut();
                    for cv in cell_vec {
                        let v = iter.next().unwrap();
                        v.sat_id = cv.0;
                        v.sig_id = if let Some(v) = to_sig(cv.1) {
                            v
                        } else {
                            return Err(RtcmError::InvalidSignalId);
                        }
                    }
                }
                $(
                    for v in value.iter_mut() {
                        v.$field_name = $frag_id::decode(par)?;
                        // v.$field_name = if let Ok(v) = $frag_id::decode(par) {
                        //     v
                        // } else {
                        //     return Err(());
                        // };
                    }
                )+
                Ok(value)
            }
            #[cfg(feature = "test_gen")]
            use $crate::val_gen::ValGen;
            #[cfg(feature = "test_gen")]
            pub fn generate<FR,LR,RR>(asm:&mut Assembler, val_gen:&mut ValGen<FR,LR,RR>, sig_len:usize) -> Result<(),RtcmError> 
            where FR:rand::Rng, LR:rand::Rng, RR:rand::Rng {
                $(
                    for _ in 0..sig_len {
                        $frag_id::generate(asm, val_gen)?;
                        // if $frag_id::generate(asm, val_gen).is_err() {
                        //     return Err(());
                        // }
                    }
                )+
                Ok(())
            }
            #[cfg(feature = "test_gen")]
            impl $crate::source_repr::SourceRepr for $type_name {
                fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    use core::fmt::Write;

                    write!(f, "{} {{", stringify!($type_name))?;
                    f.write_str("sat_id: ")?;
                    self.sat_id.to_source(f)?;
                    write!(f, ", sig_id: {}::", stringify!($gnss))?;
                    self.sig_id.to_source(f)?;
                    f.write_char(',')?;

                    $(
                        write!(f, "{}: ", stringify!($field_name))?;
                        self.$field_name.to_source(f)?;
                        f.write_char(',')?;
                    )+
                    f.write_char('}')?;

                    Ok(())
                }
            }
        }
    };
}
#[allow(unused)]
fn mask_to_id_vec_u32(mask: u32) -> ArrayVec<[u8; 32]> {
    let mut indx_vec = ArrayVec::new();
    for i in 0..32 {
        if (mask >> (31 - i)) % 2 == 1 {
            indx_vec.push(i + 1);
        }
    }
    indx_vec
}
#[allow(unused)]
fn mask_to_id_vec_u64(mask: u64) -> ArrayVec<[u8; 64]> {
    let mut indx_vec = ArrayVec::new();
    for i in 0..64 {
        if (mask >> (63 - i)) % 2 == 1 {
            indx_vec.push(i + 1);
        }
    }
    indx_vec
}
#[allow(unused)]
fn cell_mask_id_vec(
    sat_mask: u64,
    sig_mask: u32,
    cell_mask: u64,
) -> Option<(ArrayVec<[u8; 64]>, ArrayVec<[(u8, u8); 64]>)> {
    let sat_vec = mask_to_id_vec_u64(sat_mask);
    let sig_vec = mask_to_id_vec_u32(sig_mask);
    let cell_cont_len = sat_vec.len() * sig_vec.len();
    if cell_cont_len > 64 || cell_cont_len == 0 {
        return None;
    }
    let mut cell_vec = ArrayVec::new();
    for i in 0..cell_cont_len {
        if (cell_mask >> (cell_cont_len - 1 - i)) % 2 == 1 {
            cell_vec.push((sat_vec[i / sig_vec.len()], sig_vec[i % sig_vec.len()]));
        }
    }
    Some((sat_vec, cell_vec))
}
// macro_rules! pub_msg {
//     (
//         id: $id:ident,
//         pub_type: $pub_type:ident,
//         feature: $feature:literal,
//     ) => {
//         #[cfg(feature=$feature)]
//         pub mod $id;
//         #[cfg(feature=$feature)]
//         pub type $pub_type = $id::$id::DataType;
//         #[cfg(feature=$feature)]
//         impl $pub_type {
//             #[inline(always)]
//             pub fn encode(asm:&mut Assembler, value:&$pub_type) -> Result<(),()> { //remove
//                 $id::$id::encode(asm,value)
//             }
//             #[inline(always)]
//             pub fn decode(par:&mut Parser) -> Result<$pub_type,()> { //remove as it is visible
//                 $id::$id::decode(par)
//             }
//         }
//     };
// }
#[cfg(any(feature = "msg1071"))]
mod msm123_sat;
#[cfg(any(
    feature = "msg1074", 
    feature = "msg1084", 
    feature = "msg1094",
    feature = "msg1104",
    feature = "msg1114",
    feature = "msg1124"
))]
mod msm46_sat;

macro_rules! include_msg {
    ($msg:ident, $feature:literal) => {
        #[cfg(feature = $feature)]
        mod $msg;
        #[cfg(feature = $feature)]
        pub use $msg::$msg::export_types::*;
    };
}

include_msg!(msg1001, "msg1001");
include_msg!(msg1005, "msg1005");
include_msg!(msg1007, "msg1007");
include_msg!(msg1008, "msg1008");
include_msg!(msg1030, "msg1030");
include_msg!(msg1071, "msg1071");
include_msg!(msg1074, "msg1074");
include_msg!(msg1084, "msg1084");
include_msg!(msg1094, "msg1094");
include_msg!(msg1104, "msg1104");
include_msg!(msg1114, "msg1114");
include_msg!(msg1124, "msg1124");

//#[cfg(feature = "msg1001")]
//mod msg1001;
//pub use msg1001::msg1001::export_types::*;

//#[cfg(feature = "msg1005")]
//mod msg1005;
//pub use msg1005::msg1005::export_types::*;

// #[cfg(feature = "msg1007")]
// mod msg1007;
// pub use msg1007::msg1007::export_types::*;

// #[cfg(feature = "msg1008")]
// mod msg1008;
// pub use msg1008::msg1008::export_types::*;

// #[cfg(feature = "msg1030")]
// mod msg1030;
// pub use msg1030::msg1030::export_types::*;

// #[cfg(feature = "msg1071")]
// mod msg1071;
// pub use msg1071::msg1071::export_types::*;

// #[cfg(feature = "msg1074")]
// mod msg1074;
// pub use msg1074::msg1074::export_types::*;

// pub_msg!(
//     id:msg1001,
//     pub_type: Msg1001T,
//     feature: "msg1001",
// );
// pub_msg!(
//     id:msg1005,
//     pub_type: Msg1005T,
//     feature: "msg1005",
// );
// pub_msg!(
//     id:msg1007,
//     pub_type: Msg1007T,
//     feature: "msg1007",
// );
// pub_msg!(
//     id:msg1008,
//     pub_type: Msg1008T,
//     feature: "msg1008",
// );
// pub_msg!(
//     id:msg1030,
//     pub_type: Msg1030T,
//     feature: "msg1030",
// );
// pub_msg!(
//     id:msg1071,
//     pub_type: Msg1071T,
//     feature: "msg1071",
// );
// pub_msg!(
//     id:msg1074,
//     pub_type: Msg1074T,
//     feature: "msg1074",
// );
