//! Many implementations to make Stringlet easy to use.

use crate::*;

/**
Magic sauce for a UTF-8 hack: a byte containing two high bits is not a valid last byte.
Use this a a marker to distinguish whether we use the full length. Otherwise the lower bits contain
the length of the unused tail. At full length there is no tagged last byte, so we only need to encode
64 lengths. Which is where this crate’s length limit comes from.

To enable simple eq-test, always put the same value on all unused bytes! Counting from the end, i.e.
the length of the unused tail, makes the branchless implementation of `len()` more efficient.

If you change the semantics, `option_env!("STRINGLET_RAW_DEBUG")` is your friend.
*/
pub(crate) const TAG: u8 = 0b11_000000;

impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8>
    StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    pub const fn from_str(str: &str) -> Result<Self, ()> {
        if Self::fits(str.len()) {
            // SAFETY we checked the length
            Ok(unsafe { Self::from_str_unchecked(str) })
        } else {
            Err(())
        }
    }

    /// # Safety
    /// It is the callers responsibility to ensure that the size fits
    pub const unsafe fn from_str_unchecked(str: &str) -> Self {
        // SAFETY len() is up to the caller
        unsafe { Self::from_utf8_unchecked(str.as_bytes()) }
    }

    #[doc(hidden)]
    #[inline]
    pub const fn _from_macro(str: &str) -> Self {
        if Self::fits(str.len()) {
            // SAFETY checked the length and got UTF-8
            unsafe { Self::from_str_unchecked(str) }
        } else if FIXED {
            panic!("stringlet!(...): parameter too short or too long.")
        } else {
            panic!("stringlet!(var|slim ...): parameter too long.")
        }
    }

    pub fn from_utf8_bytes(str: [u8; SIZE]) -> Result<Self, std::str::Utf8Error> {
        str::from_utf8(&str)?;
        // SAFETY always short enough and just checked for UTF-8 error
        Ok(unsafe { Self::from_utf8_bytes_unchecked(str) })
    }

    /// # Safety
    /// It is the callers responsibility to ensure that the content is UTF-8.
    pub const unsafe fn from_utf8_bytes_unchecked(str: [u8; SIZE]) -> Self {
        Self {
            _align: [],
            str,
            len: [if LEN > 0 { str.len() as _ } else { 0 }; LEN],
        }
    }

    pub fn from_utf8(bytes: &[u8]) -> Result<Self, std::str::Utf8Error> {
        // todo return an Error, e.g. std::array::TryFromSliceError
        assert!(
            Self::fits(bytes.len()),
            "{}::from_utf8(): cannot store {} characters",
            std::any::type_name::<Self>(),
            bytes.len()
        );
        str::from_utf8(bytes)?;
        // SAFETY we checked the length and utf8ness
        Ok(unsafe { Self::from_utf8_unchecked(bytes) })
    }

    /// # Safety
    /// It is the callers responsibility to ensure that the size fits and the content is UTF-8.
    pub const unsafe fn from_utf8_unchecked(bytes: &[u8]) -> Self {
        let bytes_len = bytes.len();

        let mut str_uninit = core::mem::MaybeUninit::uninit();
        let str = str_uninit.as_mut_ptr() as *mut u8;

        Self {
            _align: [],
            // SAFETY we only write to uninit via pointer methods before Rust sees the value
            str: unsafe {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), str, bytes_len);
                if !FIXED {
                    let tail = if LEN == 1 {
                        TAG
                    } else {
                        TAG | (SIZE - bytes_len) as u8
                    };
                    str.add(bytes_len).write_bytes(tail, SIZE - bytes_len);
                }
                str_uninit.assume_init()
            },
            len: [bytes_len as _; _],
        }
    }

    /* Once we add appending
    pub const fn capacity(&self) -> usize {
        SIZE
    } */

    #[inline(always)]
    pub const fn len(&self) -> usize {
        if FIXED || SIZE == 0 {
            return SIZE;
        } else if LEN == 1 {
            // For VarStringlet look no further
            return self.len[0] as _;
        }

        // Only SlimStringlet after here
        let last = self.last();
        if SIZE == 1 {
            // iff single byte is untagged we have 1
            (last < TAG) as _
        } else if SIZE == 64 {
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
        } else if FIXED {
            // and already checked SIZE > 0
            false
        } else if LEN == 1 {
            // For VarStringlet look no further
            self.len[0] == 0
        } else if SIZE == 64 {
            // Special case as we have 65 lengths but only 6 bits.
            self.last() == TAG
        } else {
            self.last() == TAG | SIZE as u8
        }
    }

    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        if FIXED {
            &self.str
        } else {
            // No [..self.len()] in const yet, asm differs in debug but same as slice in release
            self.str.split_at(self.len()).0
        }
    }

    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        // SAFETY: str up to len() is guaranteed to to be initialized with valid UTF-8
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Name without StringletBase details in 3 parts, on which you must call .join("")
    pub(crate) const fn type_name() -> [&'static str; 3] {
        let name = ["SlimStringlet", "VarStringlet"][LEN];
        let log2 = ALIGN.trailing_zeros() as usize;
        let align = ["", "2", "4", "8", "16", "32", "64"][log2];
        let size = [
            "<0>", "<1>", "<2>", "<3>", "<4>", "<5>", "<6>", "<7>", "<8>", "<9>", "<10>", "<11>",
            "<12>", "<13>", "<14>", "<15>", "", "<17>", "<18>", "<19>", "<20>", "<21>", "<22>",
            "<23>", "<24>", "<25>", "<26>", "<27>", "<28>", "<29>", "<30>", "<31>", "<32>", "<33>",
            "<34>", "<35>", "<36>", "<37>", "<38>", "<39>", "<40>", "<41>", "<42>", "<43>", "<44>",
            "<45>", "<46>", "<47>", "<48>", "<49>", "<50>", "<51>", "<52>", "<53>", "<54>", "<55>",
            "<56>", "<57>", "<58>", "<59>", "<60>", "<61>", "<62>", "<63>", "<64>",
        ][SIZE];

        if FIXED {
            // Skip "Slim". No [4..] in const yet
            [name.split_at(4).1, align, size]
        } else {
            [name, align, size]
        }
    }

    #[inline(always)]
    pub(crate) const fn fits(len: usize) -> bool {
        if FIXED { len == SIZE } else { len <= SIZE }
    }

    #[inline(always)]
    pub(crate) const fn last(&self) -> u8 {
        debug_assert!(SIZE != 0, "unchecked call");
        self.str[SIZE - 1]
    }
}

#[cfg(doctest)]
mod doctests {
    /**
    ```compile_fail
    let _x: stringlet::VarStringlet<256>;
    ```
    */
    fn test_var_stringlet_256_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::SlimStringlet<65>;
    ```
    */
    fn test_slim_stringlet_65_compile_fail() {}

    /**
    ```compile_fail
    # use stringlet::{align, StringletBase};
    let _x: StringletBase::<align::Align1, 0, true, 1>;
    ```
    */
    fn test_fixed_1_compile_fail() {}
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
            unsafe { Stringlet::<4>::from_utf8_bytes_unchecked([b'A', b'b', b'c', b'd']) };
        assert_eq!(&ABCD, "Abcd");
        const A123456: Stringlet<7> = stringlet!("A123456");
        assert_eq!(&A123456, "A123456");
    }

    fn test_all_lengths<const SIZE: usize>()
    where
        Stringlet<SIZE>: Config<SIZE>,
        VarStringlet<SIZE>: Config<SIZE, false, 1>,
        SlimStringlet<SIZE>: Config<SIZE, false>,
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
                assert_eq!(stringify!($ty).replace(' ', ""), <$ty>::type_name().join(""));
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
        test_it!(SlimStringlet);
        test_it!(Stringlet2);
        test_it!(VarStringlet4);
        test_it!(SlimStringlet8);
        test_it!(Stringlet16);
        test_it!(VarStringlet32);
        test_it!(SlimStringlet64);
    }
}
