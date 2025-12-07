//! Stringlet `new()` and `default()` only where they make sence.

use crate::*;
macro_rules! new {
    ($kind:ident [$($gen:tt)*] $size:tt, $len:literal) => {
        impl<$($gen)* const ALIGN: u8> StringletBase<$kind, $size, $len, ALIGN>
        where
            Self: ConfigBase<$kind, $size, $len, ALIGN>,
        {
            pub const fn new() -> Self {
                // SAFETY always short enough and no bytes that can have a UTF-8 error
                unsafe { Self::from_utf8_unchecked(&[]) }
            }
        }

        impl<$($gen)* const ALIGN: u8> Default for StringletBase<$kind, $size, $len, ALIGN>
        where
            Self: ConfigBase<$kind, $size, $len, ALIGN>,
        {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

new! { Fixed [] 0, 0 }
new! { Trim [] 0, 0 }
new! { Trim [] 1, 0 }
new! { Var [const SIZE: usize,] SIZE, 1 }
new! { Slim [const SIZE: usize,] SIZE, 0 }

// These are twice 3x identical but for different generic constellations.
// A fixed Stringlet can only be empty for SIZE 0.
/* impl<const ALIGN: u8> StringletBase<Fixed, 0, 0, ALIGN>
where
    Self: ConfigBase<Fixed, 0, 0, ALIGN>,
{
    pub const fn new() -> Self {
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }
}

impl<const ALIGN: u8> Default for StringletBase<Fixed, 0, 0, ALIGN>
where
    Self: ConfigBase<Fixed, 0, 0, ALIGN>,
{
    fn default() -> Self {
        Self::new()
    }
}

// A variable Stringlet, slim or not, can always be empty.
impl<const SIZE: usize, const LEN: usize, const ALIGN: u8> StringletBase<SIZE, false, LEN, ALIGN>
where
    Self: ConfigBase<SIZE, false, LEN, ALIGN>,
{
    pub const fn new() -> Self {
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }
}

impl<const SIZE: usize, const LEN: usize, const ALIGN: u8> Default
    for StringletBase<SIZE, false, LEN, ALIGN>
where
    Self: ConfigBase<SIZE, false, LEN, ALIGN>,
{
    fn default() -> Self {
        Self::new()
    }
} */

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
    fn test_new() {
        let s = Stringlet::new();
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
