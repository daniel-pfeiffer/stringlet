//! Creation of stringlets, `new()` and `default()` panic at compile time, unless they make sence.

use crate::*;

impl<Kind: StringletKind, const SIZE: usize, const LEN: usize> StringletBase<Kind, SIZE, LEN>
where
    Self: ConfigBase<Kind, SIZE, LEN>,
{
    /** Create an empty `Self``. Will panic if type can’t be empty, e.g. `Stringlet<1>` or  `TrimStringlet<2>` */
    #[inline(always)]
    pub const fn new() -> Self {
        const {
            if Self::FIXED {
                assert!(SIZE == 0, "Stringlet<1> or longer cannot be empty");
            } else if Self::TRIM {
                assert!(SIZE <= 1, "TrimStringlet<2> or longer cannot be empty");
            }
        }
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }

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

    pub fn from_utf8_bytes(str: [u8; SIZE]) -> Result<Self, core::str::Utf8Error> {
        str::from_utf8(&str)?;
        // SAFETY always short enough and just checked for UTF-8 error
        Ok(unsafe { Self::from_utf8_bytes_unchecked(str) })
    }

    /// # Safety
    /// It is the callers responsibility to ensure that the content is UTF-8.
    pub const unsafe fn from_utf8_bytes_unchecked(str: [u8; SIZE]) -> Self {
        Self {
            str,
            len: [str.len() as _; _],
            _kind: PhantomData,
        }
    }

    pub fn from_utf8(bytes: &[u8]) -> Result<Self, core::str::Utf8Error> {
        // todo return an Error, e.g. core::array::TryFromSliceError
        assert!(
            Self::fits(bytes.len()),
            "{}::from_utf8(): cannot store {} characters",
            Self::type_name(),
            bytes.len()
        );
        str::from_utf8(bytes)?;
        // SAFETY we checked the length and utf8ness
        Ok(unsafe { Self::from_utf8_unchecked(bytes) })
    }

    #[doc(hidden)]
    #[inline]
    pub const fn _from_macro(str: &str) -> Self {
        if Self::fits(str.len()) {
            // SAFETY checked the length and got UTF-8
            unsafe { Self::from_utf8_unchecked(str.as_bytes()) }
        } else if Self::FIXED {
            panic!("stringlet!(...): parameter too short or too long.")
        } else if Self::TRIM {
            panic!("stringlet!(trim ...): parameter too short or too long.")
        } else {
            panic!("stringlet!(var|slim ...): parameter too long.")
        }
    }

    /// # Safety
    /// It is the callers responsibility to ensure that the size fits and the content is UTF-8.
    pub const unsafe fn from_utf8_unchecked(bytes: &[u8]) -> Self {
        let bytes_len = bytes.len();

        let mut str_uninit = core::mem::MaybeUninit::uninit();
        let str = str_uninit.as_mut_ptr() as *mut u8;

        Self {
            // SAFETY we only write to uninit via pointer methods before Rust sees the value
            str: unsafe {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), str, bytes_len);
                if !Self::FIXED {
                    let tail = TAG | (SIZE - bytes_len) as u8;
                    str.add(bytes_len).write_bytes(tail, SIZE - bytes_len);
                }
                str_uninit.assume_init()
            },
            len: [bytes_len as _; _],
            _kind: PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) const fn fits(len: usize) -> bool {
        if Self::FIXED {
            len == SIZE
        } else if Self::TRIM {
            len == SIZE || len + 1 == SIZE
        } else {
            len <= SIZE
        }
    }
}

impl_for! {
    bound Default:

    fn default() -> Self {
        Self::new()
    }
}

#[cfg(doctest)]
mod doctests {
    /**
    ```compile_fail
    _ = stringlet::Stringlet::<1>::new();
    ```
    */
    fn test_stringlet_1_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::Stringlet<1> = Default::default();
    ```
    */
    fn test_stringlet_1_default_compile_fail() {}

    /**
    ```compile_fail
    _ = stringlet::TrimStringlet::<2>::new();
    ```
    */
    fn test_trim_stringlet_2_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::TrimStringlet<2> = Default::default();
    ```
    */
    fn test_trim_stringlet_2_default_compile_fail() {}

    /**
    ```compile_fail
    _ = stringlet::VarStringlet::<256>::new();
    ```
    */
    fn test_var_stringlet_256_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::VarStringlet<256> = Default::default();
    ```
    */
    fn test_var_stringlet_256_default_compile_fail() {}

    /**
    ```compile_fail
    _ = stringlet::SlimStringlet::<65>::new();
    ```
    */
    fn test_slim_stringlet_65_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::SlimStringlet<65> = Default::default();
    ```
    */
    fn test_slim_stringlet_65_default_compile_fail() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fits() {
        assert!(Stringlet::<0>::fits(0));
        assert!(!Stringlet::<0>::fits(1));
        assert!(Stringlet::<1>::fits(1));
        assert!(!Stringlet::<1>::fits(0));
        assert!(Stringlet::<256>::fits(256));

        assert!(VarStringlet::<0>::fits(0));
        assert!(!VarStringlet::<0>::fits(1));
        assert!(VarStringlet::<1>::fits(1));
        assert!(VarStringlet::<1>::fits(0));
        assert!(VarStringlet::<2>::fits(0));
        assert!(VarStringlet::<255>::fits(0));
        assert!(VarStringlet::<255>::fits(255));

        assert!(TrimStringlet::<0>::fits(0));
        assert!(!TrimStringlet::<0>::fits(1));
        assert!(TrimStringlet::<1>::fits(0));
        assert!(TrimStringlet::<1>::fits(1));
        assert!(!TrimStringlet::<2>::fits(0));
        assert!(TrimStringlet::<256>::fits(255));
        assert!(TrimStringlet::<256>::fits(256));

        assert!(SlimStringlet::<0>::fits(0));
        assert!(!SlimStringlet::<0>::fits(1));
        assert!(SlimStringlet::<1>::fits(1));
        assert!(SlimStringlet::<1>::fits(0));
        assert!(SlimStringlet::<2>::fits(0));
        assert!(SlimStringlet::<64>::fits(0));
        assert!(SlimStringlet::<64>::fits(64));
    }

    #[test]
    fn test_new() {
        let s = Stringlet::<0>::new();
        assert!(s.is_empty());
        let s = TrimStringlet::<0>::new();
        assert!(s.is_empty());
        let s = TrimStringlet::<1>::new();
        assert!(s.is_empty());
        let s = VarStringlet::<8>::new();
        assert!(s.is_empty());
        let s = SlimStringlet::<8>::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_default() {
        let s: Stringlet<0> = Default::default();
        assert!(s.is_empty());
        let s: TrimStringlet<0> = Default::default();
        assert!(s.is_empty());
        let s: TrimStringlet<1> = Default::default();
        assert!(s.is_empty());
        let s: VarStringlet = Default::default();
        assert!(s.is_empty());
        let s: SlimStringlet = Default::default();
        assert!(s.is_empty());
    }
}
