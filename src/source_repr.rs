use core::fmt::Display;

pub struct SourceOutput<'a, S:SourceRepr> {
    object: &'a S
}

pub trait SourceRepr {
    fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

impl<S:SourceRepr> Display for SourceOutput<'_, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.object.to_source(f)
    }
}

impl<'a, S:SourceRepr> SourceOutput<'a, S> {
    pub fn new(object: &'a S) -> Self {
        SourceOutput { object }
    }
}

macro_rules! source_repr_prim {
    ($t:ty) => {
        impl SourceRepr for $t {
            fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

source_repr_prim!(u8);
source_repr_prim!(u16);
source_repr_prim!(u32);
source_repr_prim!(u64);
source_repr_prim!(i8);
source_repr_prim!(i16);
source_repr_prim!(i32);
source_repr_prim!(i64);
source_repr_prim!(usize);
source_repr_prim!(isize);
source_repr_prim!(f32);
source_repr_prim!(f64);
source_repr_prim!(char);

source_repr_prim!(Option<u8>);
source_repr_prim!(Option<u16>);
source_repr_prim!(Option<u32>);
source_repr_prim!(Option<u64>);
source_repr_prim!(Option<i8>);
source_repr_prim!(Option<i16>);
source_repr_prim!(Option<i32>);
source_repr_prim!(Option<i64>);
source_repr_prim!(Option<usize>);
source_repr_prim!(Option<isize>);
source_repr_prim!(Option<f32>);
source_repr_prim!(Option<f64>);