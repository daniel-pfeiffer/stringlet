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
pub(crate) const TAIL_TAG: u8 = 0b1100_0000;

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
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
    pub const fn _from_macro(str: &str) -> Self {
        if Self::fits(str.len()) {
            // SAFETY checked the length and got UTF-8
            unsafe { Self::from_str_unchecked(str) }
        } else if FIXED {
            panic!("stringlet!(=...): parameter too short or too long.")
        } else {
            panic!("stringlet!(...): parameter too long.")
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
        Self { str }
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
        let len = bytes.len();
        let mut result = std::mem::MaybeUninit::uninit();
        let dest = result.as_mut_ptr() as *mut u8;
        Self {
            // SAFETY we only write to uninit via pointer methods before Rust sees the value
            str: unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), dest, len);
                dest.add(len)
                    .write_bytes(TAIL_TAG | (SIZE - len) as u8, SIZE - len);
                result.assume_init()
            },
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
        }
        let last = self.last();
        if SIZE == 1 {
            // ff single byte is untagged we have 1
            (last < TAIL_TAG) as _
        } else if SIZE == 64 {
            // 64 is special as we only store 6 bits, where 0b00_0000 means SIZE-0b100_0000
            SIZE - (last == TAIL_TAG) as usize * SIZE
                - (last > TAIL_TAG) as usize * (last ^ TAIL_TAG) as usize
        } else {
            // branchless: if last is UTF-8, SIZE, else extract tail len from low bits of last
            SIZE - (last > TAIL_TAG) as usize * (last ^ TAIL_TAG) as usize
        }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        if SIZE == 0 {
            true
        } else if FIXED {
            false
        } else if SIZE == 1 {
            o!(self)[0] >= TAIL_TAG
        } else {
            let last = self.last();
            // bit-and is not short-circuiting but branchless
            (o!(self)[0] == last) & (last >= TAIL_TAG)
        }
    }

    #[inline(always)]
    pub(crate) const fn last(&self) -> u8 {
        debug_assert!(SIZE != 0, "unchecked call");
        o!(self)[SIZE - 1]
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // SAFETY: str up to len() is guaranteed to to be initialized with valid UTF-8
        unsafe {
            if FIXED {
                str::from_utf8_unchecked(&self.str)
            } else {
                str::from_utf8_unchecked(&self.str[..self.len()])
            }
        }
    }

    pub(crate) const fn type_name() -> &'static str {
        let fixed = [
            "FixedStringlet",
            "FixedStringlet2",
            "FixedStringlet4",
            "FixedStringlet8",
            "FixedStringlet16",
            "FixedStringlet32",
            "FixedStringlet64",
        ][ALIGN.trailing_zeros() as usize];
        if FIXED {
            fixed
        } else {
            // No [5..] in const yet
            let (_, flex) = fixed.split_at(5);
            flex
        }
    }

    #[inline(always)]
    pub(crate) const fn fits(len: usize) -> bool {
        if FIXED { len == SIZE } else { len <= SIZE }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        const ABCD: Stringlet<4> =
            unsafe { Stringlet::<4>::from_utf8_bytes_unchecked([b'A', b'b', b'c', b'd']) };
        assert_eq!(&ABCD, "Abcd");
        const A123456: Stringlet<7> = stringlet!("A123456");
        assert_eq!(&A123456, "A123456");
    }

    #[test]
    fn test_new() {
        stringlet!("A123456");
        let s = Stringlet::<8>::new();
        assert_eq!(&s, "");
    }

    fn test_all_lengths<const SIZE: usize>()
    where
        Stringlet<SIZE>: Config<SIZE>,
    {
        const STR64: &str = "0123456789_123456789_123456789_123456789_123456789_123456789_123";
        for len in 0..=SIZE {
            let str: Stringlet<SIZE> = (&STR64[..len]).into();
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
        test_all_lengths!(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
        );
    }

    #[test]
    fn test_empty() {
        assert!(stringlet!("").is_empty());
        assert!(stringlet!(1: "").is_empty());
        assert!(stringlet!(2: "").is_empty());
        assert!(!stringlet!("a").is_empty());
        assert!(!stringlet!("ab").is_empty());
        assert!(stringlet!(="").is_empty());
        assert!(!stringlet!(="a").is_empty());
        assert!(!stringlet!(="ab").is_empty());
    }
}
