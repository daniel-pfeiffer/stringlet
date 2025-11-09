//! Many implementations to make Stringlet easy to use.

use crate::*;

use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> From<String>
    for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn from(str: String) -> Self {
        Self::from(str.as_str())
    }
}

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> From<&str>
    for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn from(str: &str) -> Self {
        assert!(
            Self::fits(str.len()),
            "{}::from(): cannot store {} characters",
            Self::type_name(),
            str.len()
        );
        // SAFETY we checked the length and str is UTF-8
        unsafe { Self::from_utf8_unchecked(str.as_bytes()) }
    }
}

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> std::str::FromStr
    for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> Hash for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        o!(self).hash(state);
    }
}

impl<
    const SIZE: usize,
    const SIZE2: usize,
    const FIXED: bool,
    const FIXED2: bool,
    const ALIGN: u8,
    const ALIGN2: u8,
> PartialEq<Stringlet<SIZE2, FIXED2, ALIGN2>> for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
    Stringlet<SIZE2, FIXED2, ALIGN2>: Config<SIZE2, ALIGN2>,
{
    fn eq(&self, other: &Stringlet<SIZE2, FIXED2, ALIGN2>) -> bool {
        match (SIZE == SIZE2, FIXED, FIXED2) {
            // Slice only needed because the compiler can’t reason about these types being same.
            // Would need specialization, with either both being SIZE or SIZE != SIZE2.
            (true, ..) => o!(self) == o!(other)[..],
            // Fixeds can only be same at same length.
            (_, true, true) => false,
            // Either fixed one doesn’t need a dynamic slice
            (_, true, _) => o!(self) == o!(other)[..other.len()],
            (.., true) => o!(self)[..self.len()] == o!(other),
            _ => o!(self)[..self.len()] == o!(other)[..other.len()],
        }
    }
}

// Gnats: Ord is inconsistent with PartialEq, in that I can only compare to Self
impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> Ord for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if FIXED {
            o!(self).cmp(&o!(other))
        } else {
            o!(self)[..self.len()].cmp(&o!(other)[..other.len()])
        }
    }
}

// Why can’t this be derived from Ord?
impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> PartialOrd
    for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> PartialEq<str>
    for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        // todo When is this precondition beneficial or harmful?
        self.len() == other.len() && self.as_str() == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_eq() {
        macro_rules! assert_eq_all {
            ($a:expr) => {};
            ($a:expr, $b:expr $(, $($rest:tt)*)?) => {
                assert_eq!($a, $b);
                assert_eq_all!($b $(, $($rest)*)?)
            };
        }

        let s_x_1 = Stringlet::<1>::from("x");
        let s_x_2 = Stringlet::<1>::from("x");
        let s_y = Stringlet::<1>::from("y");
        let s2_x = Stringlet::<2>::from("x");
        let s2_y = Stringlet::<2>::from("y");
        let s2_xy = Stringlet::<2>::from("xy");

        let f_x_1 = FixedStringlet::<1>::from("x");
        let f_x_2 = FixedStringlet::<1>::from("x");
        let f_y = FixedStringlet::<1>::from("y");
        let f2_xy = FixedStringlet::<2>::from("xy");

        assert_eq_all!(s_x_1, s_x_2, s2_x, f_x_1, f_x_2);
        assert_eq_all!(s_y, s2_y, f_y);
        assert_eq!(s2_xy, f2_xy);

        assert_ne!(s_x_1, s_y);
        assert_ne!(s_x_1, s2_y);
        assert_ne!(s2_y, s_x_1);
        assert_ne!(s_x_1, s2_xy);
        assert_ne!(s2_xy, s_x_1);

        assert_ne!(s_x_1, f_y);
        assert_ne!(s_x_1, f2_xy);
        assert_ne!(f2_xy, s_x_1);
    }
}
