/*
macro_rules! sig_id_impl {
    ($type_name:ident) => {
        impl $type_name {
            pub fn new(band: u8, attribute: char) -> $type_name {
                $type_name(band, attribute)
            }
            pub fn band(self) -> u8 {
                self.0
            }
            pub fn attribute(self) -> char {
                self.1
            }
            pub fn is_valid(self) -> bool {
                to_id(self).is_some()
            }
        }
        #[cfg(feature = "test_gen")]
        impl $crate::source_repr::SourceRepr for $type_name {
            fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use core::fmt::Write;
                write!(f, "{}::new(", stringify!($type_name))?;
                self.0.to_source(f)?;
                f.write_char(',')?;
                self.1.to_source(f)?;
                f.write_char(')')?;

                Ok(())
            }
        }
    };
}  */

macro_rules! msm_mappings {
    (
        gnss: $gnss:ident,
        mappings: [ $( $num:literal => $band:literal|$attr:literal ),+ ]
    ) => {
        pub mod $gnss {
            #[cfg(feature = "serde")]
            use $crate::{Deserialize, Serialize};
            #[cfg(feature = "serde")]
            extern crate sd;

            #[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
            pub struct SigId(u8, char);

            impl SigId {
                pub fn new(band: u8, attribute: char) -> SigId {
                    SigId(band, attribute)
                }
                pub fn band(self) -> u8 {
                    self.0
                }
                pub fn attribute(self) -> char {
                    self.1
                }
                pub fn is_valid(self) -> bool {
                    to_id(self).is_some()
                }
            }

            pub fn to_sig(id: u8) -> Option<SigId> {
                match id {
                    $(
                        $num => Some(SigId($band,$attr)),
                    )+
                    _ => None,
                }
            }

            pub fn to_id(sig: SigId) -> Option<u8> {
                match sig {
                    $(
                        SigId($band, $attr) => Some($num),
                    )+
                    _ => None,
                }
            }

            #[cfg(feature = "test_gen")]
            impl $crate::source_repr::SourceRepr for SigId {
                fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    use core::fmt::Write;
                    write!(f, "{}::new(", stringify!(SigId))?;
                    self.0.to_source(f)?;
                    f.write_char(',')?;
                    self.1.to_source(f)?;
                    f.write_char(')')?;

                    Ok(())
                }
            }

            #[cfg(feature = "test_gen")]
            pub fn random_id<R: rand::Rng + ?Sized>(rng: &mut R, subset:usize) -> u8 {
                const IDS:&[u8] = &[ $($num),+ ];
                let id_idx:usize = ((subset % 32) + (rng.gen::<usize>() % 8)) % IDS.len();
                IDS[id_idx]
            }
            impl core::cmp::PartialOrd for SigId {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    let l = to_id(*self);
                    let r = to_id(*other);
                    if let (Some(l), Some(r)) = (l, r) {
                        return l.partial_cmp(&r);
                    } else {
                        None
                    }
                }
            }
            impl core::cmp::Ord for SigId {
                fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                    let l = to_id(*self);
                    let r = to_id(*other);
                    if let (Some(l), Some(r)) = (l, r) {
                        return l.cmp(&r);
                    }
                    if let (None, Some(_)) = (l, r) {
                        return core::cmp::Ordering::Greater;
                    }
                    if let (Some(_), None) = (l, r) {
                        return core::cmp::Ordering::Less;
                    }
                    match self.0.cmp(&other.0) {
                        core::cmp::Ordering::Less => core::cmp::Ordering::Less,
                        core::cmp::Ordering::Equal => self.1.cmp(&other.1),
                        core::cmp::Ordering::Greater => core::cmp::Ordering::Greater,
                    }
                }
            }
        }
    };
}

msm_mappings!(
    gnss: gps,
    mappings: [
        2 => 1|'C',
        3 => 1|'P',
        4 => 1|'W',
        8 => 2|'C',
        9 => 2|'P',
        10 => 2|'W',
        15 => 2|'S',
        16 => 2|'L',
        17 => 2|'X',
        22 => 5|'I',
        23 => 5|'Q',
        24 => 5|'X',
        30 => 1|'S',
        31 => 1|'L',
        32 => 1|'X'
    ]
);
msm_mappings!(
    gnss: glo,
    mappings: [
        2 => 1|'C',
        3 => 1|'P',
        8 => 2|'C',
        9 => 2|'P'
    ]
);
msm_mappings!(
    gnss: gal,
    mappings: [
        2 => 1|'C',
        3 => 1|'A',
        4 => 1|'B',
        5 => 1|'X',
        6 => 1|'Z',
        8 => 6|'C',
        9 => 6|'A',
        10 => 6|'B',
        11 => 6|'X',
        12 => 6|'Z',
        14 => 7|'I',
        15 => 7|'Q',
        16 => 7|'X',
        18 => 8|'I',
        19 => 8|'Q',
        20 => 8|'X',
        22 => 1|'I',
        23 => 1|'Q',
        24 => 1|'X'
    ]
);
msm_mappings!(
    gnss: bds,
    mappings: [
        2 => 2|'I',
        3 => 2|'Q',
        4 => 2|'X',
        8 => 6|'I',
        9 => 6|'Q',
        10 => 6|'X',
        14 => 7|'I',
        15 => 7|'Q',
        16 => 7|'X'

    ]
);

/*
pub mod gps {
    #[cfg(feature = "serde")]
    use crate::{Deserialize, Serialize};
    #[cfg(feature = "serde")]
    extern crate sd;

    #[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "sd"))]
    pub struct SigId(u8, char);

    sig_id_impl!(SigId);

    pub fn to_sig(id: u8) -> Option<SigId> {
        match id {
            2 => Some(SigId(1, 'C')),
            3 => Some(SigId(1, 'P')),
            4 => Some(SigId(1, 'W')),
            8 => Some(SigId(2, 'C')),
            9 => Some(SigId(2, 'P')),
            10 => Some(SigId(2, 'W')),
            15 => Some(SigId(2, 'S')),
            16 => Some(SigId(2, 'L')),
            17 => Some(SigId(2, 'X')),
            22 => Some(SigId(5, 'I')),
            23 => Some(SigId(5, 'Q')),
            24 => Some(SigId(5, 'X')),
            30 => Some(SigId(1, 'S')),
            31 => Some(SigId(1, 'L')),
            32 => Some(SigId(1, 'X')),
            _ => None,
        }
    }
    pub fn to_id(sig: SigId) -> Option<u8> {
        match sig {
            SigId(1, 'C') => Some(2),
            SigId(1, 'P') => Some(3),
            SigId(1, 'W') => Some(4),
            SigId(2, 'C') => Some(8),
            SigId(2, 'P') => Some(9),
            SigId(2, 'W') => Some(10),
            SigId(2, 'S') => Some(15),
            SigId(2, 'L') => Some(16),
            SigId(2, 'X') => Some(17),
            SigId(5, 'I') => Some(22),
            SigId(5, 'Q') => Some(23),
            SigId(5, 'X') => Some(24),
            SigId(1, 'S') => Some(30),
            SigId(1, 'L') => Some(31),
            SigId(1, 'X') => Some(32),
            _ => None,
        }
    }
    #[cfg(feature = "test_gen")]
    pub fn random_id<R: rand::Rng + ?Sized>(rng: &mut R) -> u8 {
        let mut id: u8 = (rng.gen::<u8>() % 32) + 1;
        while to_sig(id).is_none() {
            id = rng.gen();
        }
        id
    }
    impl core::cmp::PartialOrd for SigId {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            let l = to_id(*self);
            let r = to_id(*other);
            if let (Some(l), Some(r)) = (l, r) {
                return l.partial_cmp(&r);
            } else {
                None
            }
        }
    }
    impl core::cmp::Ord for SigId {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            let l = to_id(*self);
            let r = to_id(*other);
            if let (Some(l), Some(r)) = (l, r) {
                return l.cmp(&r);
            }
            if let (None, Some(_)) = (l, r) {
                return core::cmp::Ordering::Greater;
            }
            if let (Some(_), None) = (l, r) {
                return core::cmp::Ordering::Less;
            }
            match self.0.cmp(&other.0) {
                core::cmp::Ordering::Less => core::cmp::Ordering::Less,
                core::cmp::Ordering::Equal => self.1.cmp(&other.1),
                core::cmp::Ordering::Greater => core::cmp::Ordering::Greater,
            }
        }
    }
}
 */
/*
pub mod glo {
    #[cfg(feature = "serde_derive")]
    use crate::{Deserialize, Serialize};

    #[derive(Clone, Copy, PartialEq, Default, Debug)]
    #[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
    pub struct SigId(u8, char);

    sig_id_impl!(SigId);

    pub fn to_sig(id: u8) -> Option<SigId> {
        match id {
            _ => None,
        }
    }
    pub fn to_id(sig: SigId) -> Option<u8> {
        match sig {
            _ => None,
        }
    }
    #[cfg(feature = "test_gen")]
    pub fn random_id<R: rand::Rng + ?Sized>(rng: &mut R) -> u8 {
        let mut id: u8 = rng.gen();
        while to_sig(id).is_none() {
            id = rng.gen();
        }
        id
    }
    impl core::cmp::PartialOrd for SigId {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            let l = to_id(*self);
            let r = to_id(*other);
            if let (Some(l), Some(r)) = (l, r) {
                return l.partial_cmp(&r);
            } else {
                None
            }
        }
    }
}

pub mod gal {
    #[cfg(feature = "serde_derive")]
    use crate::{Deserialize, Serialize};

    #[derive(Clone, Copy, PartialEq, Default, Debug)]
    #[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
    pub struct SigId(u8, char);

    sig_id_impl!(SigId);

    pub fn to_sig(id: u8) -> Option<SigId> {
        match id {
            _ => None,
        }
    }
    pub fn to_id(sig: SigId) -> Option<u8> {
        match sig {
            _ => None,
        }
    }
    #[cfg(feature = "test_gen")]
    pub fn random_id<R: rand::Rng + ?Sized>(rng: &mut R) -> u8 {
        let mut id: u8 = rng.gen();
        while to_sig(id).is_none() {
            id = rng.gen();
        }
        id
    }
    impl core::cmp::PartialOrd for SigId {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            let l = to_id(*self);
            let r = to_id(*other);
            if let (Some(l), Some(r)) = (l, r) {
                return l.partial_cmp(&r);
            } else {
                None
            }
        }
    }
}

pub mod bds {
    #[cfg(feature = "serde_derive")]
    use crate::{Deserialize, Serialize};

    #[derive(Clone, Copy, PartialEq, Default, Debug)]
    #[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
    pub struct SigId(u8, char);

    sig_id_impl!(SigId);

    pub fn to_sig(id: u8) -> Option<SigId> {
        match id {
            _ => None,
        }
    }
    pub fn to_id(sig: SigId) -> Option<u8> {
        match sig {
            _ => None,
        }
    }
    #[cfg(feature = "test_gen")]
    pub fn random_id<R: rand::Rng + ?Sized>(rng: &mut R) -> u8 {
        let mut id: u8 = rng.gen();
        while to_sig(id).is_none() {
            id = rng.gen();
        }
        id
    }
    impl core::cmp::PartialOrd for SigId {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            let l = to_id(*self);
            let r = to_id(*other);
            if let (Some(l), Some(r)) = (l, r) {
                return l.partial_cmp(&r);
            } else {
                None
            }
        }
    }
}
*/
