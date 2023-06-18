use tinyvec::ArrayVec;

mod array_string;
mod data_vec;

#[cfg(feature = "serde")]
use crate::{Deserialize, Serialize, Visitor};
use array_string::ArrayString;
use core::{fmt::Write, slice::Iter};
pub use data_vec::DataVec;

#[derive(Clone, PartialEq)]
pub struct Df88591String<const N: usize>(ArrayVec<[u8; N]>);
impl<const N: usize> Df88591String<N> {
    pub fn new() -> Self {
        Df88591String(ArrayVec::new())
    }
    pub fn chars(&self) -> Df88591StringChars<'_> {
        Df88591StringChars {
            iter: self.0.iter(),
        }
    }
    pub fn iter(&self) -> Iter<'_, u8> {
        self.0.iter()
    }
    pub fn push(&mut self, val: u8) {
        self.0.push(if val == 0 { 0xa4 } else { val });
    }
    pub fn push_char(&mut self, ch: char) {
        self.0.push(Df88591StringChars::<'_>::from_char(ch));
    }
    #[inline]
    pub fn try_push(&mut self, ch: char) -> Result<(), ()> {
        if self.0.len() + 1 > self.0.capacity() {
            return Err(());
        }
        self.0.push(Df88591StringChars::<'_>::from_char(ch));
        Ok(())
    }
}
impl<const N: usize> From<&str> for Df88591String<N> {
    fn from(value: &str) -> Self {
        value.chars().collect()
    }
}
impl<const N: usize> FromIterator<char> for Df88591String<N> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Df88591String<N> {
        let mut buf = Df88591String::new();
        for c in iter.into_iter() {
            if buf.try_push(c).is_err() {
                break;
            }
        }
        buf
    }
}
impl<const N: usize> Default for Df88591String<N> {
    fn default() -> Self {
        Df88591String::new()
    }
}
impl<const N: usize> core::fmt::Display for Df88591String<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        //use core::fmt::Write;
        write!(f, "Df88591String<{}>(\"", N)?;
        for c in self.chars() {
            f.write_char(c)?;
        }
        f.write_str("\")")
    }
}
impl<const N: usize> core::fmt::Debug for Df88591String<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val: ArrayVec<[char; N]> = self
            .0
            .iter()
            .map(|v| Df88591StringChars::to_char(*v))
            .collect();
        f.debug_tuple("Df88591String").field(&val).finish()
    }
}

pub struct Df88591StringChars<'a> {
    iter: core::slice::Iter<'a, u8>,
}

impl Df88591StringChars<'_> {
    fn to_char(code: u8) -> char {
        if code == 0 {
            //Character 0x00 not allowed for Rtcm
            char::from_u32(0xa4).unwrap()
        } else {
            char::from_u32(code as u32).unwrap()
        }
    }
    fn from_char(ch: char) -> u8 {
        let code = ch as u32;
        if code > 0 && code < 256 {
            code as u8
        } else {
            0xa4
        }
    }
}

impl<'a> Iterator for Df88591StringChars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        Some(Df88591StringChars::to_char(*self.iter.next()?))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.iter.len();
        (len, Some(len))
    }
}
#[cfg(feature = "serde")]
impl<const N: usize> Serialize for Df88591String<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: sd::Serializer,
    {
        let value: ArrayString<N> = self.chars().collect();

        serializer.serialize_str(&value)
    }
}
#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for Df88591String<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: sd::Deserializer<'de>,
    {
        struct Str88591Visitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for Str88591Visitor<N> {
            type Value = Df88591String<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("an ISO 8859-1 encoded string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                let mut value = Df88591String::<N>::new();
                for ch in v.chars().take(N) {
                    value.push_char(ch);
                }
                Ok(value)
            }
        }

        deserializer.deserialize_str(Str88591Visitor::<N>)
    }
}

#[cfg(feature = "test_gen")]
use crate::source_repr::SourceRepr;

#[cfg(feature = "test_gen")]
impl<const N: usize> SourceRepr for Df88591String<N> {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = self.chars().collect::<ArrayString<N>>();
        write!(f, "Df88591String::<{}>::from(\"{}\")", N, s.as_ref())
    }
}

/*
pub struct DfString<const N: usize>(ArrayVec<[u8; N]>);
*/

/*
#[cfg(test)]
mod util_tests {
    use super::Df88591String;
}
 */
