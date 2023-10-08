#[cfg(feature = "serde")]
use crate::{Deserialize, Serialize};
#[allow(unused)]
use core::fmt::Write;
use core::ops::{Deref, DerefMut};
use core::slice::{Iter, IterMut};

#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
pub struct Grid16P<T: Default + Clone>([T; 16]);

impl<T: Default + Clone> Deref for Grid16P<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}
impl<T: Default + Clone> DerefMut for Grid16P<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[..]
    }
}

impl<T: Default + Clone> Grid16P<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Grid16P(Default::default())
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
    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.0.iter_mut()
    }
}

#[cfg(feature = "test_gen")]
use crate::source_repr::SourceRepr;
#[cfg(feature = "test_gen")]
impl<T: Default + Clone + SourceRepr> SourceRepr for Grid16P<T> {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('[')?;
        for v in self.0.iter() {
            v.to_source(f)?;
            f.write_str(",")?;
        }
        f.write_char(']')
    }
}
