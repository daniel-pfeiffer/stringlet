//! Many implementations to make Stringlet easy to use.

use crate::*;

use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> From<String>
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    fn from(str: String) -> Self {
        Self::from(str.as_str())
    }
}

impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> From<&str>
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    fn from(str: &str) -> Self {
        assert!(
            Self::fits(str.len()),
            "{}::from(): cannot store {} characters",
            Self::type_name().join(""),
            str.len()
        );
        // SAFETY we checked the length and str is UTF-8
        unsafe { Self::from_utf8_unchecked(str.as_bytes()) }
    }
}

impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> std::str::FromStr
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> Hash
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
    }
}

// todo how can I write the Rhs type only once?
impl<
    const SIZE: usize,
    const SIZE2: usize,
    const FIXED: bool,
    const FIXED2: bool,
    const LEN: usize,
    const LEN2: usize,
    const ALIGN: u8,
    const ALIGN2: u8,
> PartialEq<StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>> for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
    StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>: Config<SIZE2, FIXED2, LEN2, ALIGN2>,
{
    fn eq(&self, other: &StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>) -> bool {
        match (SIZE == SIZE2, FIXED, FIXED2) {
            // Slice only needed because the compiler can’t reason about these types being same.
            // Would need specialization, with either both being SIZE or SIZE != SIZE2.
            (true, ..) => SIZE == 0 || self.str == other.str[..],

            // Fixeds can only be same at same length.
            (_, true, true) => false,

            // Either fixed one can only be eq if shorter; and it doesn’t need a dynamic slice
            (_, true, _) => SIZE < SIZE2 && self.str == other.str[..other.len()],
            (.., true) => SIZE > SIZE2 && self.str[..self.len()] == other.str,
            _ => self.str[..self.len()] == other.str[..other.len()],
        }
    }
}

// todo validate whether calling .eq(*other) is zero cost
impl<
    'a,
    const SIZE: usize,
    const SIZE2: usize,
    const FIXED: bool,
    const FIXED2: bool,
    const LEN: usize,
    const LEN2: usize,
    const ALIGN: u8,
    const ALIGN2: u8,
> PartialEq<&'a StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>>
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
    StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>: Config<SIZE2, FIXED2, LEN2, ALIGN2>,
{
    fn eq(&self, other: &&'a StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>) -> bool {
        match (SIZE == SIZE2, FIXED, FIXED2) {
            // Slice only needed because the compiler can’t reason about these types being same.
            // Would need specialization, with either both being SIZE or SIZE != SIZE2.
            (true, ..) => SIZE == 0 || self.str == other.str[..],

            // Fixeds can only be same at same length.
            (_, true, true) => false,

            // Either fixed one can only be eq if shorter; and it doesn’t need a dynamic slice
            (_, true, _) => SIZE < SIZE2 && self.str == other.str[..other.len()],
            (.., true) => SIZE > SIZE2 && self.str[..self.len()] == other.str,
            _ => self.str[..self.len()] == other.str[..other.len()],
        }
    }
}

impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> PartialEq<str>
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        if SIZE == 0 {
            other.is_empty()
        } else if FIXED {
            self.str == *other.as_bytes()
        } else {
            self.str[..self.len()] == *other.as_bytes()
        }
    }
}

impl<'a, const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> PartialEq<&'a str>
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    #[inline(always)]
    fn eq(&self, other: &&'a str) -> bool {
        if SIZE == 0 {
            other.is_empty()
        } else if FIXED {
            self.str == *other.as_bytes()
        } else {
            self.str[..self.len()] == *other.as_bytes()
        }
    }
}

// Gnats: Ord falls short of PartialEq, in that I can only compare to Self
impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> Ord
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if FIXED {
            self.str.cmp(&other.str)
        } else {
            self.str[..self.len()].cmp(&other.str[..other.len()])
        }
    }
}

// Why can’t this be derived from Ord?
impl<const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8> PartialOrd
    for StringletBase<SIZE, FIXED, LEN, ALIGN>
where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let s: SlimStringlet<4> = String::from("hey").into();
        assert_eq!(s.as_ref(), "hey");
    }

    #[test]
    fn test_from_long_str() {
        let s: VarStringlet<16> = "Rustacean".into();
        assert_eq!(&s, "Rustacean");
    }

    #[test]
    #[should_panic]
    fn test_panics_when_too_long() {
        let _s: VarStringlet<2> = "hello world".into();
    }

    #[test]
    fn test_from_str() {
        let s = SlimStringlet::<8>::from("hello");
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

        let s_x_1 = VarStringlet::<1>::from("x");
        let s_x_2 = VarStringlet::<1>::from("x");
        let s_y = VarStringlet::<1>::from("y");
        let s2_x = VarStringlet::<2>::from("x");
        let s2_y = VarStringlet::<2>::from("y");
        let s2_xy = VarStringlet::<2>::from("xy");

        let f_x_1 = Stringlet::<1>::from("x");
        let f_x_2 = Stringlet::<1>::from("x");
        let f_y = Stringlet::<1>::from("y");
        let f2_xy = Stringlet::<2>::from("xy");

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
