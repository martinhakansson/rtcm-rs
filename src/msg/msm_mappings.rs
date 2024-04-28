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
            #[allow(unused)]
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
                let id_idx:usize = ((subset % 32) + (rng.gen::<usize>() % id_len_div64())) % IDS.len();
                IDS[id_idx]
            }
            #[cfg(feature = "test_gen")]
            pub fn id_len_div64() -> usize {
                let len = (&[ $($num),+ ]).len();
                if len == 32 { return 32; }
                if len >= 16 { return 16; }
                if len >= 8 { return 8; }
                if len >= 4 { return 4; }
                if len >= 2 { return 2; }
                return 1;
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
        22 => 5|'I',
        23 => 5|'Q',
        24 => 5|'X'
    ]
);

msm_mappings!(
    gnss: sbas,
    mappings: [
        2 => 1|'C',
        22 => 5|'I',
        23 => 5|'Q',
        24 => 5|'X'

    ]
);

msm_mappings!(
    gnss: qzss,
    mappings: [
        2 => 1|'C',
        9 => 6|'S',
        10 => 6|'L',
        11 => 6|'X',
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
        16 => 7|'X',
        22 => 5|'D',
        23 => 5|'P',
        24 => 5|'X',
        25 => 7|'D',
        30 => 1|'D',
        31 => 1|'P',
        32 => 1|'X'
    ]
);

msm_mappings!(
    gnss: navic,
    mappings: [
        8 => 9|'A',
        22 => 5|'A'
    ]
);
