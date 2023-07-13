use crate::tinyvec::ArrayVec;
#[cfg(feature = "serde")]
use crate::{Deserialize, Serialize};
#[allow(unused)]
use core::fmt::Write;
use core::ops::{Deref, DerefMut};
use core::slice::{Iter, IterMut};

#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
pub struct DataVec<T: Default + Clone, const N: usize>(ArrayVec<[T; N]>);

impl<T: Default + Clone, const N: usize> Deref for DataVec<T, N> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl<T: Default + Clone, const N: usize> DerefMut for DataVec<T, N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<T: Default + Clone, const N: usize> DataVec<T, N> {
    #[inline(always)]
    pub fn new() -> Self {
        DataVec(ArrayVec::new())
    }
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.deref()
    }
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.deref_mut()
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear();
    }
    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.0.iter_mut()
    }
    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    #[inline(always)]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }
    #[inline(always)]
    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }
    #[inline(always)]
    pub fn set_len(&mut self, new_len: usize) {
        self.0.set_len(new_len);
    }
}

#[cfg(feature = "test_gen")]
use crate::source_repr::SourceRepr;
#[cfg(feature = "test_gen")]
impl<T: Default + Clone + SourceRepr, const N: usize> SourceRepr for DataVec<T, N> {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('{')?;
        write!(
            f,
            "#[allow(unused_mut)] let mut vec = DataVec::<_,{}>::new();",
            N
        )?;
        for v in self.0.iter() {
            f.write_str("vec.push(")?;
            v.to_source(f)?;
            f.write_str(");")?;
        }
        f.write_str("vec")?;
        f.write_char('}')
    }
}
