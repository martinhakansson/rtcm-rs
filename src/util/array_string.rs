use crate::tinyvec::ArrayVec;
#[cfg(feature = "serde")]
use crate::{Deserialize, Serialize, Visitor};
use core::ops::Deref;

#[derive(Default, Clone, PartialEq)]
pub struct ArrayString<const N: usize> {
    vec: ArrayVec<[u8; N]>,
}

impl<const N: usize> ArrayString<N> {
    pub fn new() -> Self {
        let vec = ArrayVec::<[u8; N]>::new();
        ArrayString { vec }
    }
    #[inline]
    pub fn try_push(&mut self, ch: char) -> Result<(), ()> {
        let len = ch.len_utf8();
        if self.vec.len() + len > self.vec.capacity() {
            return Err(());
        }
        match len {
            1 => self.vec.push(ch as u8),
            _ => self
                .vec
                .extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()),
        }
        Ok(())
    }
}

impl<const N: usize> From<&str> for ArrayString<N> {
    fn from(value: &str) -> Self {
        let mut array_string = ArrayString::<N>::new();
        for ch in value.chars() {
            if array_string.try_push(ch).is_err() {
                break;
            }
        }
        array_string
    }
}

impl<const N: usize> Deref for ArrayString<N> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        core::str::from_utf8(&self.vec).unwrap()
    }
}

impl<const N: usize> AsRef<str> for ArrayString<N> {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const N: usize> FromIterator<char> for ArrayString<N> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> ArrayString<N> {
        let mut buf = ArrayString::new();
        for c in iter.into_iter() {
            if buf.try_push(c).is_err() {
                break;
            }
        }
        buf
    }
}

impl<const N: usize> core::fmt::Display for ArrayString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ArrayString<{}>(\"", N)?;
        f.write_str(&*self)?;
        f.write_str("\")")
    }
}
impl<const N: usize> core::fmt::Debug for ArrayString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ArrayString<{}>(\"", N)?;
        f.write_str(&*self)?;
        f.write_str("\")")
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for ArrayString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: sd::Serializer,
    {
        serializer.serialize_str(&*self)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for ArrayString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: sd::Deserializer<'de>,
    {
        struct ArrayStringVisitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for ArrayStringVisitor<N> {
            type Value = ArrayString<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("an UTF-8 encoded string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                let mut value = ArrayString::<N>::new();
                for ch in v.chars() {
                    if value.try_push(ch).is_err() {
                        break;
                    }
                }
                Ok(value)
            }
        }

        deserializer.deserialize_str(ArrayStringVisitor::<N>)
    }
}

#[cfg(feature = "test_gen")]
use crate::source_repr::SourceRepr;

#[cfg(feature = "test_gen")]
impl<const N: usize> SourceRepr for ArrayString<N> {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ArrayString::<{}>::from(\"{}\")", N, self.as_ref())
    }
}
