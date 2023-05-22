use core::{
    fmt::Debug,
    ops::{BitOrAssign, Shl, Shr},
};

pub trait BitValue
where
    Self::ValueType: Default
        + Copy
        + Debug
        + Shr<usize, Output = Self::ValueType>
        + Shl<usize, Output = Self::ValueType>
        + BitOrAssign<Self::ValueType>,
{
    //const BITS: usize;
    //const MAX: Self::ValueType;
    type ValueType;
    fn u8_cast(val: u8) -> Self::ValueType;
    fn val_cast(val: Self::ValueType) -> u8;
    fn sign_fix(val: Self::ValueType, len: usize) -> Self::ValueType;
    fn sign_fix_rev(val: Self::ValueType, len: usize) -> Self::ValueType;
}

macro_rules! u8_cast {
    () => {
        #[inline]
        fn u8_cast(val: u8) -> Self::ValueType {
            val as Self::ValueType
        }
    };
}
macro_rules! val_cast {
    () => {
        #[inline]
        fn val_cast(val: Self::ValueType) -> u8 {
            val as u8
        }
    };
}
macro_rules! impl_bit_value {
    ($mtype:ident, $ptype:ty, u) => {
        pub struct $mtype;
        impl BitValue for $mtype {
            //const BITS: usize = <$ptype>::BITS as usize;
            type ValueType = $ptype;
            u8_cast!();
            val_cast!();
            #[inline]
            fn sign_fix(val: Self::ValueType, _len: usize) -> Self::ValueType {
                val
            }
            #[inline]
            fn sign_fix_rev(val: Self::ValueType, _len: usize) -> Self::ValueType {
                val
            }
        }
    };
    ($mtype:ident, $ptype:ty, s) => {
        pub struct $mtype;
        impl BitValue for $mtype {
            //const BITS: usize = <$ptype>::BITS as usize;
            type ValueType = $ptype;
            u8_cast!();
            val_cast!();
            #[inline]
            fn sign_fix(val: Self::ValueType, len: usize) -> Self::ValueType {
                if val & (1 << (len - 1)) == 0 {
                    val
                } else {
                    val | (-1 << len)
                }
            }
            #[inline]
            fn sign_fix_rev(val: Self::ValueType, _len: usize) -> Self::ValueType {
                val
            }
        }
    };
    ($mtype:ident, $ptype:ty, sm) => {
        pub struct $mtype;
        impl BitValue for $mtype {
            //const BITS: usize = <$ptype>::BITS as usize;
            type ValueType = $ptype;
            u8_cast!();
            val_cast!();
            #[inline]
            fn sign_fix(val: Self::ValueType, len: usize) -> Self::ValueType {
                if val & (1 << (len - 1)) == 0 {
                    val
                } else {
                    //println!("shift={}", (Self::BITS - len + 1));
                    //println!("valshift={}", (-1 >> (Self::BITS - len + 1)));
                    -1 * (val & (!(-1 << (len - 1))))
                }
            }
            #[inline]
            fn sign_fix_rev(val: Self::ValueType, len: usize) -> Self::ValueType {
                if val & (1 << (len - 1)) == 0 {
                    val
                } else {
                    ((!val) + 1) | (1 << (len - 1))
                }
            }
        }
    };
}

impl_bit_value!(U8, u8, u);
impl_bit_value!(U16, u16, u);
impl_bit_value!(U32, u32, u);
impl_bit_value!(U64, u64, u);
impl_bit_value!(U128, u128, u);
impl_bit_value!(I8, i8, s);
impl_bit_value!(I16, i16, s);
impl_bit_value!(I32, i32, s);
impl_bit_value!(I64, i64, s);
impl_bit_value!(I128, i128, s);
impl_bit_value!(SM8, i8, sm);
impl_bit_value!(SM16, i16, sm);
impl_bit_value!(SM32, i32, sm);
impl_bit_value!(SM64, i64, sm);
impl_bit_value!(SM128, i128, sm);
