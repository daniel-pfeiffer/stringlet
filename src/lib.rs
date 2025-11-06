#![doc = include_str!("../README.md")]

// todo more efficient starts_with, contains, combined Length&Utf8Error instead of panic

mod fmt;
mod impl_for;
mod macros;
mod repr;
pub use repr::*;


/**
Magic sauce for a UTF-8 hack: a byte containing these bits is not a valid last byte.
Use this a a marker to distinguish whether we use the full length. Otherwise the lower bits contain
the length of the unused tail. To enable int eq-test, always put the same value on all unused bytes!

These methods of Stringlet must agree on the semantics: `from_utf8_unchecked()`, `last()` & `len()`.
Counting from the end, i.e. the length of the unused tail, makes the branchless implementation of `len()` more efficient.
For cheap `Eq` across different `CAPACITY` in same `SIZE`, we should count from the end of `raw`.
But, due to that being behind a trait, `const last()` can’t yet index into it.
So, instead count from the end of `str`.

If you change the semantics, `option_env!("STRINGLET_RAW_DEBUG")` is your friend.
*/
pub(crate) const TAIL_TAG: u8 = 0b1100_0000;

/// An inline String up to 64 bytes, which can be handled like a primitive type.
#[derive(Copy, Clone, Eq)]
pub union Stringlet<const CAPACITY: usize = 16, const FIXED: bool = false>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    /// repr is a number or tuple of numbers for cheap operations
    repr: Repr<CAPACITY>,
    /// raw is a byte array of the same size as repr, which must be fully written
    raw: Raw<CAPACITY>,
    /// str is the actual payload, which can be shorter than raw, in addition to its last bytes possibly being tagged
    str: [u8; CAPACITY],
}

impl<const CAPACITY: usize, const FIXED: bool> Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    const SIZE: usize = size_of::<Raw<CAPACITY>>();

    pub const fn new() -> Self {
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }

    pub const fn from_str(str: &str) -> Result<Self, ()> {
        if str.len() > CAPACITY {
            Err(())
        } else {
            // SAFETY we checked the length
            Ok(unsafe { Self::from_str_unchecked(str) })
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
        // SAFETY the stringlet!() macro calls this on Stringlet<str.len()> so it won’t compile if too long
        unsafe { Self::from_utf8_unchecked(str.as_bytes()) }
    }

    pub fn from_utf8_bytes(str: [u8; CAPACITY]) -> Result<Self, std::str::Utf8Error> {
        str::from_utf8(&str)?;
        // SAFETY always short enough and just checked for UTF-8 error
        Ok(unsafe { Self::from_utf8_bytes_unchecked(str) })
    }

    /// # Safety
    /// It is the callers responsibility to ensure that the content is UTF-8.
    pub const unsafe fn from_utf8_bytes_unchecked(str: [u8; CAPACITY]) -> Self {
        if CAPACITY == Self::SIZE {
            Self { str }
        } else {
            // SAFETY always short enough and no bytes that can have a UTF-8 error
            unsafe { Self::from_utf8_unchecked(&str) }
        }
    }

    pub fn from_utf8(bytes: &[u8]) -> Result<Self, std::str::Utf8Error> {
        // todo return an Error, e.g. std::array::TryFromSliceError
        assert!(
            bytes.len() <= CAPACITY,
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
        // SAFETY we only write to uninit via pointer methods before Rust sees the value
        Self {
            raw: unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), dest, len);
                dest.add(len)
                    .write_bytes(TAIL_TAG | (CAPACITY - len) as u8, Self::SIZE - len);
                result.assume_init()
            },
        }
    }
    /* pub unsafe fn from_utf8_unchecked(bytes: &[u8]) -> Self {
        let mut new = Self { int: T::default() };
        // SAFETY is up to the caller
        unsafe {
            new.str[..bytes.len()].clone_from_slice(bytes);
            // NOP if len == CAPACITY
            new.str[bytes.len()..].fill(LEN_TAG | bytes.len() as u8);
        }
        new
    } */

    pub const fn capacity(&self) -> usize {
        CAPACITY
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        if FIXED || CAPACITY == 0 {
            CAPACITY
        } else {
            let last = self.last();
            // len 64 is special as we only store 6 bits, where 0b00_0000 means 0b100_0000
            if CAPACITY == 64 {
                CAPACITY
                    - (last == TAIL_TAG) as usize * CAPACITY
                    - (last > TAIL_TAG) as usize * (last ^ TAIL_TAG) as usize
            } else {
                // branchless: if last is UTF-8, capacity, else extract len from low bits of last
                CAPACITY - (last > TAIL_TAG) as usize * (last ^ TAIL_TAG) as usize
            }
        }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        CAPACITY == 0
            || (! FIXED &&
                // SAFETY: str is guaranteed to be initialized
                unsafe { self.str[0] } >= TAIL_TAG)
    }

    #[inline(always)]
    const fn last(&self) -> u8 {
        debug_assert!(CAPACITY != 0, "unchecked call");
        // SAFETY: str is guaranteed to be initialized
        unsafe { self.str[CAPACITY - 1] }
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

    #[test]
    fn test_from_string() {
        let s: Stringlet<4> = String::from("hey").into();
        assert_eq!(s.as_ref(), "hey");
    }

    #[test]
    fn test_from_long_str() {
        let s: Stringlet<16> = "Rustacean".into();
        assert_eq!(&s, "Rustacean");
    }

    #[test]
    #[should_panic]
    fn test_panics_when_too_long() {
        let _s: Stringlet<2> = "hello world".into();
    }

    #[test]
    fn test_from_str() {
        let s = Stringlet::<8>::from("hello");
        assert_eq!(s.as_ref(), "hello");
    }

    #[test]
    fn test_deref() {
        let s = Stringlet::<4>::from("Abc");
        assert!(s.contains('b'));
    }

    #[test]
    fn test_as_ref() {
        let s = Stringlet::<1>::from("A");
        let s: &str = s.as_ref();
        assert_eq!(s, "A");
    }

    // Somehow this didn’t work as a simple generic fn.
    impl<const CAPACITY: usize, const FIXED: bool> Stringlet<CAPACITY, FIXED>
    where
        super::Size<CAPACITY>: super::Config<CAPACITY>,
    {
        fn test_all_lengths() {
            let str64 = "0123456789_123456789_123456789_123456789_123456789_123456789_123";
            for len in 0..=CAPACITY {
                let str: Stringlet<CAPACITY> = (&str64[..len]).into();
                assert_eq!(str.is_empty(), len == 0);
                assert_eq!(str.len(), len);
            }
            let fixed: Stringlet<CAPACITY> = (&str64[..CAPACITY]).into();
            assert_eq!(fixed.is_empty(), CAPACITY == 0);
            assert_eq!(fixed.len(), CAPACITY);
        }
    }

    #[test]
    fn test_len() {
        Stringlet::<0>::test_all_lengths();
        Stringlet::<1>::test_all_lengths();
        Stringlet::<2>::test_all_lengths();
        Stringlet::<3>::test_all_lengths();
        Stringlet::<4>::test_all_lengths();
        Stringlet::<5>::test_all_lengths();
        Stringlet::<6>::test_all_lengths();
        Stringlet::<7>::test_all_lengths();
        Stringlet::<8>::test_all_lengths();
        Stringlet::<9>::test_all_lengths();
        Stringlet::<10>::test_all_lengths();
        Stringlet::<11>::test_all_lengths();
        Stringlet::<12>::test_all_lengths();
        Stringlet::<13>::test_all_lengths();
        Stringlet::<14>::test_all_lengths();
        Stringlet::<15>::test_all_lengths();
        Stringlet::<16>::test_all_lengths();
        #[cfg(feature = "len64")]
        {
            Stringlet::<17>::test_all_lengths();
            Stringlet::<18>::test_all_lengths();
            Stringlet::<19>::test_all_lengths();
            Stringlet::<20>::test_all_lengths();
            Stringlet::<21>::test_all_lengths();
            Stringlet::<22>::test_all_lengths();
            Stringlet::<23>::test_all_lengths();
            Stringlet::<24>::test_all_lengths();
            Stringlet::<25>::test_all_lengths();
            Stringlet::<26>::test_all_lengths();
            Stringlet::<27>::test_all_lengths();
            Stringlet::<28>::test_all_lengths();
            Stringlet::<29>::test_all_lengths();
            Stringlet::<30>::test_all_lengths();
            Stringlet::<31>::test_all_lengths();
            Stringlet::<32>::test_all_lengths();
            Stringlet::<33>::test_all_lengths();
            Stringlet::<34>::test_all_lengths();
            Stringlet::<35>::test_all_lengths();
            Stringlet::<36>::test_all_lengths();
            Stringlet::<37>::test_all_lengths();
            Stringlet::<38>::test_all_lengths();
            Stringlet::<39>::test_all_lengths();
            Stringlet::<40>::test_all_lengths();
            Stringlet::<41>::test_all_lengths();
            Stringlet::<42>::test_all_lengths();
            Stringlet::<43>::test_all_lengths();
            Stringlet::<44>::test_all_lengths();
            Stringlet::<45>::test_all_lengths();
            Stringlet::<46>::test_all_lengths();
            Stringlet::<47>::test_all_lengths();
            Stringlet::<48>::test_all_lengths();
            Stringlet::<49>::test_all_lengths();
            Stringlet::<50>::test_all_lengths();
            Stringlet::<51>::test_all_lengths();
            Stringlet::<52>::test_all_lengths();
            Stringlet::<53>::test_all_lengths();
            Stringlet::<54>::test_all_lengths();
            Stringlet::<55>::test_all_lengths();
            Stringlet::<56>::test_all_lengths();
            Stringlet::<57>::test_all_lengths();
            Stringlet::<58>::test_all_lengths();
            Stringlet::<59>::test_all_lengths();
            Stringlet::<60>::test_all_lengths();
            Stringlet::<61>::test_all_lengths();
            Stringlet::<62>::test_all_lengths();
            Stringlet::<63>::test_all_lengths();
            Stringlet::<64>::test_all_lengths();
        }
    }
}
