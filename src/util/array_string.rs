use crate::tinyvec::ArrayVec;
use core::ops::Deref;

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
