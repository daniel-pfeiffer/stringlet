//! `Eq` & `Ord` implementations

use crate::*;

use core::cmp::Ordering;

// Needed where explicitly requested, e.g. HashMap key
impl_for! { Eq }

impl_for! {
    <2> PartialEq<StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>>:

    #[inline]
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

impl_for! {
    <'a, 2> PartialEq<&'a StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>>:

    #[inline(always)]
    fn eq(&self, other: &&'a StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>) -> bool {
        self.eq(*other)
    }
}

impl_for! {
    PartialEq<str>:

    #[inline]
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

impl_for! {
    <'a> PartialEq<&'a str>:

    #[inline(always)]
    fn eq(&self, other: &&'a str) -> bool {
        self.eq(*other)
    }
}

// Gnats: Ord falls short of PartialEq, in that I can only compare to Self
/* impl_for! {
    Ord:

    fn cmp(&self, other: &Self) -> Ordering {
        if FIXED {
            self.str.cmp(&other.str)
        } else {
            self.str[..self.len()].cmp(&other.str[..other.len()])
        }
    }
}

// Why can’t this be derived from Ord?
impl_for! {
    PartialOrd:

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
} */

impl_for! {
    <2> PartialOrd<StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>>:

    fn partial_cmp(&self, other: &StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>) -> Option<Ordering> {
        Some(if FIXED && FIXED2 {
            self.str[..].cmp(&other.str[..])
        } else if FIXED {
            self.str[..].cmp(&other.str[..other.len()])
        } else {
            self.str[..self.len()].cmp(&other.str[..other.len()])
        })
    }
}

impl_for! {
    <'a, 2> PartialOrd<&'a StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>>:

    fn partial_cmp(&self, other: &&'a StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

// Needed where explicitly requested, e.g. BTreeMap key
impl_for! {
    Ord:

    fn cmp(&self, other: &Self) -> Ordering {
        // Safe to unwrap, as we always return Some.
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
