//! Creation of stringlets, `new()` and `default()` panic at compile time, unless they make sence.

use crate::*;

impl<Kind: StringletKind, const SIZE: usize> StringletBase<Kind, SIZE>
where
    Self: ConfigBase<Kind, SIZE>,
{
    /** Create an empty `Self`. Will panic if type can’t be empty, e.g. `Stringlet<1>` or  `TrimStringlet<2>` */
    #[inline(always)]
    #[must_use]
    pub const fn new() -> Self {
        const {
            if Kind::FIXED {
                assert!(SIZE == 0, "Stringlet<1> or bigger cannot be empty");
            } else if Kind::TRIM {
                assert!(SIZE <= 1, "TrimStringlet<2> or bigger cannot be empty");
            }
        }
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }

    pub const fn from_str(str: &str) -> Result<Self> {
        match Self::fits(str.len()) {
            Ok(()) => {
                // SAFETY we checked the length and got UTF-8
                Ok(unsafe { Self::from_str_unchecked(str) })
            }
            Err(e) => Err(e),
        }
    }

    /// # Safety
    /// It is the callers responsibility to ensure that the size fits
    pub const unsafe fn from_str_unchecked(str: &str) -> Self {
        // SAFETY len() is up to the caller
        unsafe { Self::from_utf8_unchecked(str.as_bytes()) }
    }

    /**
    ```
    # use stringlet::{Stringlet, Result};
    let abcd = unsafe { Stringlet::<4>::from_utf8(b"Abcd") }?;
    assert_eq!(abcd, "Abcd");
    assert!(Stringlet::<0>::from_utf8(b"A").is_err());
    assert!(Stringlet::<1>::from_utf8(b"").is_err());
    assert!(Stringlet::<1>::from_utf8(b"\xFF").is_err());
    # Result::Ok(())
    ```
    */
    pub const fn from_utf8(str: &[u8]) -> Result<Self> {
        // const equivalent of `expr?`
        match Self::fits(str.len()) {
            Ok(()) => {
                match str::from_utf8(str) {
                    // SAFETY always short enough and no bytes have a UTF-8 error
                    Ok(str) => Ok(unsafe { Self::from_str_unchecked(str) }),
                    Err(e) => Err(Utf8Error(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    /**
    ```
    # use stringlet::Stringlet;
    const ABCD: Stringlet<4> =
        unsafe { Stringlet::from_utf8_unchecked(b"Abcd") };
    assert_eq!(ABCD, "Abcd");
    ```
    # Safety
    It is the callers responsibility to ensure that the size fits and the content is UTF-8. */
    pub const unsafe fn from_utf8_unchecked(str: &[u8]) -> Self {
        let bytes_len = str.len();

        /* let mut str_uninit = core::mem::MaybeUninit::uninit();
        let str = str_uninit.as_mut_ptr() as *mut u8; */
        let mut me_uninit = core::mem::MaybeUninit::<Self>::uninit();
        let me = me_uninit.as_mut_ptr() as *mut u8;
        // SAFETY we write to whole uninit via pointer methods only before Rust sees the value
        unsafe {
            core::ptr::copy_nonoverlapping(str.as_ptr(), me, bytes_len);
            if Kind::VAR {
                me.add(bytes_len).write_bytes(0, SIZE - bytes_len);
                me.add(SIZE).write(bytes_len as _);
            } else if !Kind::FIXED && SIZE > 0 && SIZE > bytes_len {
                me.add(bytes_len).write_bytes(0, SIZE - bytes_len - 1);
                let tail = TAG | (SIZE - bytes_len) as u8;
                me.add(SIZE - 1).write(tail);
            }
            me_uninit.assume_init()
        }
    }

    /**
    ```
    # use stringlet::{Stringlet, Result};
    let abcd = Stringlet::from_utf8_bytes([b'A', b'b', b'c', b'd'])?;
    assert_eq!(abcd, "Abcd");
    assert!(Stringlet::from_utf8_bytes([0xFF]).is_err());
    # Result::Ok(())
    ```
    */
    pub const fn from_utf8_bytes(str: [u8; SIZE]) -> Result<Self> {
        // const equivalent of `expr?`
        match str::from_utf8(&str) {
            // SAFETY always short enough and no bytes have a UTF-8 error
            Ok(str) => Ok(unsafe { Self::from_str_unchecked(str) }),
            Err(e) => Err(Utf8Error(e)),
        }
    }

    /**
    ```
    # use stringlet::Stringlet;
    # unsafe { Stringlet::from_utf8_bytes_unchecked([]) }; // for llvm-cov
    const ABCD: Stringlet<4> =
        unsafe { Stringlet::from_utf8_bytes_unchecked([b'A', b'b', b'c', b'd']) };
    assert_eq!(ABCD, "Abcd");
    ```
    # Safety
    It is the callers responsibility to ensure that the content is UTF-8. */
    pub const unsafe fn from_utf8_bytes_unchecked(str: [u8; SIZE]) -> Self {
        // SAFETY It is the callers responsibility to ensure that the content is UTF-8.
        unsafe { Self::from_utf8_unchecked(&str) }
    }

    /**
    ```
    # use stringlet::{Stringlet, Result};
    let abcd = Stringlet::from_utf8_slice(b"Abcd")?;
    assert_eq!(abcd, "Abcd");
    assert!(Stringlet::from_utf8_slice(b"\xFF").is_err());
    # Result::Ok(())
    ```
    */
    pub const fn from_utf8_slice(str: &[u8; SIZE]) -> Result<Self> {
        // const equivalent of `expr?`
        match str::from_utf8(str) {
            // SAFETY always short enough and no slice have a UTF-8 error
            Ok(str) => Ok(unsafe { Self::from_str_unchecked(str) }),
            Err(e) => Err(Utf8Error(e)),
        }
    }

    /**
    ```
    # use stringlet::Stringlet;
    # unsafe { Stringlet::from_utf8_slice_unchecked(b"") }; // for llvm-cov
    const ABCD: Stringlet<4> =
        unsafe { Stringlet::from_utf8_slice_unchecked(b"Abcd") };
    assert_eq!(ABCD, "Abcd");
    ```
    # Safety
    It is the callers responsibility to ensure that the content is UTF-8. */
    pub const unsafe fn from_utf8_slice_unchecked(str: &[u8; SIZE]) -> Self {
        // SAFETY It is the callers responsibility to ensure that the content is UTF-8.
        unsafe { Self::from_utf8_unchecked(str) }
    }

    #[doc(hidden)]
    #[inline]
    pub const fn _from_macro(str: &str) -> Self {
        match Self::fits(str.len()) {
            // SAFETY we checked the length and got UTF-8
            Ok(()) => unsafe { Self::from_str_unchecked(str) },
            Err(TooLong) => panic!("stringlet!(...): parameter too long for its type."),
            Err(TooShort) => panic!("stringlet!(...): parameter too short for its type."),
            Err(Utf8Error(_)) => unreachable!(),
        }
    }

    pub(crate) const fn fits(len: usize) -> Result<()> {
        if len > SIZE {
            Err(TooLong)
        } else if (Kind::FIXED && len == SIZE)
            || Kind::VAR
            || Kind::SLIM
            || (Kind::TRIM && len >= const { SIZE.saturating_sub(1) })
        {
            Ok(())
        } else {
            Err(TooShort)
        }
    }
}

impl_for! {
    Default:

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
    fn stringlet_1_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::Stringlet<1> = Default::default();
    ```
    */
    fn stringlet_1_default_compile_fail() {}

    /**
    ```compile_fail
    _ = stringlet::TrimStringlet::<2>::new();
    ```
    */
    fn trim_stringlet_2_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::TrimStringlet<2> = Default::default();
    ```
    */
    fn trim_stringlet_2_default_compile_fail() {}

    /**
    ```compile_fail
    _ = stringlet::VarStringlet::<256>::new();
    ```
    */
    fn var_stringlet_256_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::VarStringlet<256> = Default::default();
    ```
    */
    fn var_stringlet_256_default_compile_fail() {}

    /**
    ```compile_fail
    _ = stringlet::SlimStringlet::<65>::new();
    ```
    */
    fn slim_stringlet_65_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::SlimStringlet<65> = Default::default();
    ```
    */
    fn slim_stringlet_65_default_compile_fail() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits() {
        assert!(Stringlet::<0>::fits(0).is_ok());
        assert!(Stringlet::<0>::fits(1).is_err());
        assert!(Stringlet::<1>::fits(1).is_ok());
        assert!(Stringlet::<1>::fits(0).is_err());
        assert!(Stringlet::<256>::fits(256).is_ok());

        assert!(VarStringlet::<0>::fits(0).is_ok());
        assert!(!VarStringlet::<0>::fits(1).is_ok());
        assert!(VarStringlet::<1>::fits(1).is_ok());
        assert!(VarStringlet::<1>::fits(0).is_ok());
        assert!(VarStringlet::<2>::fits(0).is_ok());
        assert!(VarStringlet::<255>::fits(0).is_ok());
        assert!(VarStringlet::<255>::fits(255).is_ok());

        assert!(TrimStringlet::<0>::fits(0).is_ok());
        assert!(!TrimStringlet::<0>::fits(1).is_ok());
        assert!(TrimStringlet::<1>::fits(0).is_ok());
        assert!(TrimStringlet::<1>::fits(1).is_ok());
        assert!(!TrimStringlet::<2>::fits(0).is_ok());
        assert!(TrimStringlet::<256>::fits(255).is_ok());
        assert!(TrimStringlet::<256>::fits(256).is_ok());

        assert!(SlimStringlet::<0>::fits(0).is_ok());
        assert!(!SlimStringlet::<0>::fits(1).is_ok());
        assert!(SlimStringlet::<1>::fits(1).is_ok());
        assert!(SlimStringlet::<1>::fits(0).is_ok());
        assert!(SlimStringlet::<2>::fits(0).is_ok());
        assert!(SlimStringlet::<64>::fits(0).is_ok());
        assert!(SlimStringlet::<64>::fits(64).is_ok());
    }

    #[test]
    fn new() {
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
    fn default() {
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
