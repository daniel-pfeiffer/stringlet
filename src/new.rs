//! Stringlet `new()` and `default()` only where they make sence.

use crate::*;

// These are twice 3x identical but for different generic constellations.
// A fixed Stringlet can only be empty for SIZE 0.
impl<const ALIGN: u8> StringletBase<0, true, 0, ALIGN>
where
    Self: Config<0, true, 0, ALIGN>,
{
    pub const fn new() -> Self {
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }
}

impl<const ALIGN: u8> Default for StringletBase<0, true, 0, ALIGN>
where
    Self: Config<0, true, 0, ALIGN>,
{
    fn default() -> Self {
        Self::new()
    }
}

// A variable Stringlet, slim or not, can always be empty.
impl<const SIZE: usize, const LEN: usize, const ALIGN: u8> StringletBase<SIZE, false, LEN, ALIGN>
where
    Self: Config<SIZE, false, LEN, ALIGN>,
{
    pub const fn new() -> Self {
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }
}

impl<const SIZE: usize, const LEN: usize, const ALIGN: u8> Default
    for StringletBase<SIZE, false, LEN, ALIGN>
where
    Self: Config<SIZE, false, LEN, ALIGN>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(doctest)]
mod doctests {
    /**
    ```compile_fail
    let _x = stringlet::Stringlet::<5>::new();
    ```
    */
    fn test_stringlet_5_new_compile_fail() {}

    /**
    ```compile_fail
    let _x: stringlet::Stringlet<5> = Default::default();
    ```
    */
    fn test_stringlet_5_default_compile_fail() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Stringlet::<0>::new();
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
        let s: VarStringlet = Default::default();
        assert!(s.is_empty());
        let s: SlimStringlet = Default::default();
        assert!(s.is_empty());
    }
}
