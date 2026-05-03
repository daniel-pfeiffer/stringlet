//! Many implementations to make stringlet easy to use.

use crate::*;

impl<Kind: StringletKind, const SIZE: usize> StringletBase<Kind, SIZE> {
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        if Kind::FIXED {
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

    /* Once we add appending
    pub const fn capacity(&self) -> usize {
        SIZE
    } */

    #[inline(always)]
    pub const fn len(&self) -> usize {
        // optimizer should elide all but one if-branch as all conditions are const
        if Kind::FIXED || SIZE == 0 {
            return SIZE;
        }

        let last = self.last();
        if Kind::VAR {
            // For VarStringlet look no further
            last as _
        } else if SIZE == 1 {
            // iff single byte is not TAG | 1 we have 1 (use +, because mutants flags ^, which in this case is identical to |)
            (last != TAG + 1) as _
        } else if Kind::TRIM {
            // branchless: if last is tagged, subtract one
            SIZE - (last == TAG + 1) as usize
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
            true
        } else if Kind::FIXED {
            // FIXED > 0 can never be empty
            false
        } else if Kind::VAR {
            self.last() == 0
        } else if Kind::TRIM {
            // TRIM is only empty if last is tagged
            SIZE == 1 && self.last() == TAG + 1
        }
        // Next two SlimStringlet
        else if SIZE == 64 {
            // Special case as we have 65 lengths but only 6 bits.
            self.last() == TAG
        } else {
            self.last() == TAG + SIZE as u8
        }
    }

    #[inline(always)]
    pub(crate) const fn last(&self) -> u8 {
        if Kind::VAR {
            self.var_last()
        } else {
            debug_assert!(SIZE != 0, "unchecked call");
            self.str[SIZE - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big() {
        let _f: Stringlet<1024>;
        let _v: VarStringlet<255>;
        let _s: SlimStringlet<64>;
    }

    #[test]
    fn test_as_str() -> Result<()> {
        let f: Stringlet<7> = "A123456".try_into()?;
        assert_eq!(f.as_str(), "A123456");
        let v: VarStringlet = "A123456".try_into()?;
        assert_eq!(v.as_str(), "A123456");
        let s: SlimStringlet = "A123456".try_into()?;
        assert_eq!(s.as_str(), "A123456");
        Ok(())
    }

    fn test_all_lengths<const SIZE: usize>()
    where
        VarStringlet<SIZE>: VarConfig<SIZE>,
        SlimStringlet<SIZE>: SlimConfig<SIZE>,
    {
        let str64s: [&str; 3] = [
            "0123456789_123456789_123456789_123456789_123456789_123456789_123",
            str::from_utf8(&[0; 64]).unwrap(),
            str::from_utf8(&[0x7f_u8; 64]).unwrap(),
        ];
        let fixed: Stringlet<SIZE> = (&str64s[0][..SIZE]).try_into().unwrap();
        assert_eq!(fixed.is_empty(), SIZE == 0);
        assert_eq!(fixed.len(), SIZE);
        for len in 0..=SIZE {
            let str: VarStringlet<SIZE> = (&str64s[0][..len]).try_into().unwrap();
            assert_eq!(str.is_empty(), len == 0);
            assert_eq!(str.len(), len);
            for str64 in str64s {
                if len >= const { SIZE.saturating_sub(1) } {
                    let str: TrimStringlet<SIZE> = (&str64[..len]).try_into().unwrap();
                    assert_eq!(str.is_empty(), len == 0);
                    assert_eq!(str.len(), len);
                }
                let str: SlimStringlet<SIZE> = (&str64[..len]).try_into().unwrap();
                assert_eq!(str.is_empty(), len == 0);
                assert_eq!(str.len(), len);
            }
        }
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
}
