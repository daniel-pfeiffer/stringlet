//! Many implementations to make stringlet easy to use.

use crate::*;

macro_rules! consts {
    ($($fn:ident -> $const:ident);+ $(;)?) => {
        $(
            #[allow(unused)]
            pub(crate) const $const: bool =
                Kind::ABBR == stringify!($const).as_bytes()[0];
            #[allow(unused)]
            pub(crate) const fn $fn(&self) -> bool {
                Self::$const
            }
        )+
    };
}

impl<Kind: StringletKind, const SIZE: usize, const LEN: usize> StringletBase<Kind, SIZE, LEN> {
    consts! {
        is_fixed -> FIXED;
        is_trim -> TRIM;
        is_var -> VAR;
        is_slim -> SLIM;
    }

    /* Once we add appending
    pub const fn capacity(&self) -> usize {
        SIZE
    } */

    #[inline(always)]
    pub const fn len(&self) -> usize {
        // optimizer should elide all but one if-branch
        if Self::FIXED || SIZE == 0 {
            return SIZE;
        } else if Self::VAR {
            // For VarStringlet look no further
            return self.len[0] as _;
        }

        let last = self.last();
        if SIZE == 1 {
            // iff single byte is untagged we have 1
            (last < TAG) as _
        } else if Self::TRIM {
            // branchless: if last is tagged, subtract one
            SIZE - (last > TAG) as usize
        }
        // Next two SlimStringlet
        else if SIZE == 64 {
            // 64 is special as we only store 6 bits, where 0b00_0000 means SIZE-0b100_0000
            SIZE - (last == TAG) as usize * SIZE - (last > TAG) as usize * (last ^ TAG) as usize
        } else {
            // branchless: if last is UTF-8, SIZE, else extract tail len from low bits of last
            SIZE - (last > TAG) as usize * (last ^ TAG) as usize
        }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        if SIZE == 0 {
            // trivially
            return true;
        }

        let last = self.last();
        if Self::FIXED {
            // and already checked SIZE > 0
            false
        } else if Self::TRIM {
            SIZE == 1 && last > TAG
        } else if Self::VAR {
            self.len[0] == 0
        }
        // Only SlimStringlet after here
        else if SIZE == 64 {
            // Special case as we have 65 lengths but only 6 bits.
            last == TAG
        } else {
            last == TAG | SIZE as u8
        }
    }

    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        if Self::FIXED {
            &self.str
        } else {
            // const equivalent of [..self.len()], asm differs in debug but same as slice in release
            //self.str.split_at(self.len()).0
            // SAFETY This is what String aka Vec uses and all bytes are initialized. This is 30% faster than split_at.
            unsafe { core::slice::from_raw_parts(self.str.as_ptr(), self.len()) }
        }
    }

    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        // SAFETY: str up to len() is guaranteed to to be initialized with valid UTF-8
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Name for now as a String
    pub(crate) fn type_name() -> String {
        // todo longest: TrimStringlet<usize::MAX> -> VarStringlet<35>
        use core::fmt::Write;
        let mut ret = String::with_capacity(20) // mostly enough
            + if Self::FIXED {
                "Stringlet"
            } else if Self::VAR {
                "VarStringlet"
            } else if Self::TRIM {
                "TrimStringlet"
            } else {
                "SlimStringlet"
            };
        if SIZE != 16 {
            _ = write!(ret, "<{}>", SIZE);
        }
        ret
    }

    #[inline(always)]
    pub(crate) const fn last(&self) -> u8 {
        debug_assert!(SIZE != 0, "unchecked call");
        self.str[SIZE - 1]
    }
}

#[cfg(test)]
mod tests {
    use core::convert::Into;

    use super::*;

    #[test]
    fn test_big() {
        let _f: Stringlet<1024>;
        let _v: VarStringlet<255>;
        let _s: SlimStringlet<64>;
    }

    #[test]
    fn test_as_str() {
        let f: Stringlet<7> = "A123456".into();
        assert_eq!(f.as_str(), "A123456");
        let v: VarStringlet = "A123456".into();
        assert_eq!(v.as_str(), "A123456");
        let s: SlimStringlet = "A123456".into();
        assert_eq!(s.as_str(), "A123456");
    }

    #[test]
    fn test_const() {
        const ABCD: Stringlet<4> =
            unsafe { Stringlet::<_>::from_utf8_bytes_unchecked([b'A', b'b', b'c', b'd']) };
        assert_eq!(&ABCD, "Abcd");
        const A123456: Stringlet<7> = stringlet!("A123456");
        assert_eq!(&A123456, "A123456");
    }

    fn test_all_lengths<const SIZE: usize>()
    where
        VarStringlet<SIZE>: VarConfig<SIZE>,
        SlimStringlet<SIZE>: SlimConfig<SIZE>,
    {
        const STR64: &str = "0123456789_123456789_123456789_123456789_123456789_123456789_123";
        for len in 0..=SIZE {
            let str: VarStringlet<SIZE> = (&STR64[..len]).into();
            assert_eq!(str.is_empty(), len == 0);
            assert_eq!(str.len(), len);
            let str: SlimStringlet<SIZE> = (&STR64[..len]).into();
            assert_eq!(str.is_empty(), len == 0);
            assert_eq!(str.len(), len);
        }
        let fixed: Stringlet<SIZE> = (&STR64[..SIZE]).into();
        assert_eq!(fixed.is_empty(), SIZE == 0);
        assert_eq!(fixed.len(), SIZE);
        let trim: TrimStringlet<SIZE> = (&STR64[..SIZE]).into();
        assert_eq!(trim.is_empty(), SIZE == 0);
        assert_eq!(trim.len(), SIZE);
    }
    #[test]
    fn test_len() {
        macro_rules! test_all_lengths {
            ($($size:literal),+) => {
                $(test_all_lengths::<$size>();)+
            };
        }
        test_all_lengths![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
        ];
    }

    #[test]
    fn test_empty() {
        assert!(stringlet!("").is_empty());
        assert!(!stringlet!("a").is_empty());
        assert!(!stringlet!("ab").is_empty());

        assert!(stringlet!(trim: "").is_empty());
        assert!(stringlet!(trim 1: "").is_empty());
        assert!(!stringlet!(trim: "a").is_empty());
        assert!(!stringlet!(trim 2: "a").is_empty());
        assert!(!stringlet!(trim: "ab").is_empty());
        assert!(!stringlet!(trim 3: "ab").is_empty());

        assert!(stringlet!(var: "").is_empty());
        assert!(stringlet!(var 1: "").is_empty());
        assert!(stringlet!(var 2: "").is_empty());
        assert!(!stringlet!(var: "a").is_empty());
        assert!(!stringlet!(var: "ab").is_empty());

        assert!(stringlet!(slim: "").is_empty());
        assert!(stringlet!(slim 1: "").is_empty());
        assert!(stringlet!(slim 2: "").is_empty());
        assert!(!stringlet!(slim: "a").is_empty());
        assert!(!stringlet!(slim: "ab").is_empty());
    }

    #[test]
    fn test_all_type_names() {
        macro_rules! test_it {
            (1 $ty:ty) => {
                assert_eq!(<$ty>::type_name(), stringify!($ty).replace(' ', ""));
            };
            ([$($size:literal),+] $ty:tt) => {
                $(
                    test_it!(1 $ty<$size>);
                )+
            };
            ($ty:tt) => {
                test_it!(1 $ty); // special case default 16
                test_it!([
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23,
                    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                    44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
                ] $ty);
            };
        }
        test_it!(Stringlet);
        test_it!(VarStringlet);
        test_it!(TrimStringlet);
        test_it!(SlimStringlet);
    }
}
