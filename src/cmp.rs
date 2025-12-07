//! `Eq` & `Ord` implementations

use crate::*;

use core::cmp::Ordering;

// Needed where explicitly requested, e.g. HashMap key
impl_for! { Eq }

impl_for! {
    <2> PartialEq<self2!()>:

    #[inline]
    fn eq(&self, other: &self2!()) -> bool {
        // Successively eliminate cases that can be excluded at compile time, based on sizes and kinds.
        // All kinds get filled up with the same tagged value, so often no need to calculate len.
        // This favors comparing full arrays, hopefully by SIMD, rather than calculating the slices.
        match (SIZE == SIZE2, Self::FIXED, other.is_fixed(), Self::TRIM, other.is_trim()) {
            // Slice only needed because the compiler can’t reason about these types being same.
            // Would need specialization, with either both being SIZE or SIZE != SIZE2.
            (true, ..) => SIZE == 0 || self.str == other.str[..],

            // Else fixeds can not be same.
            (_, true, true, ..) => false,

            // Else either fixed can only be eq to trim if shorter by 1; and it doesn’t need a dynamic slice
            (_, true, _, _, true) => SIZE < SIZE2 && SIZE + 1 == SIZE2 && self.str == other.as_bytes(),
            (_, _, true, true, _) => SIZE > SIZE2 && SIZE == SIZE2 + 1 && self.as_bytes() == other.str,

            // Else either fixed can only be eq if shorter; and it doesn’t need a dynamic slice
            (_, true, ..) => SIZE < SIZE2 && self.str == other.as_bytes(),
            (_, _, true, ..) => SIZE > SIZE2 && self.as_bytes() == other.str,

            // Else either trim can only be eq if shorter or longer by 1
            (.., true, _) => (SIZE < SIZE2 || SIZE == SIZE2 + 1) && self.as_bytes() == other.as_bytes(),
            (.., true) => (SIZE > SIZE2 || SIZE + 1 == SIZE2) && self.as_bytes() == other.as_bytes(),

            // Else must do full cmp with dynamic lengths
            _ => self.as_bytes() == other.as_bytes(),
        }
    }
}

impl_for! {
    <'a, 2> PartialEq<&'a self2!()>:

    #[inline(always)]
    fn eq(&self, other: &&'a self2!()) -> bool {
        self.eq(*other)
    }
}

impl_for! {
    PartialEq<str>:

    #[inline]
    fn eq(&self, other: &str) -> bool {
        if SIZE == 0 {
            other.is_empty()
        } else if Self::FIXED {
            self.str == other.as_bytes()
        } else {
            self.as_bytes() == other.as_bytes()
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

impl_for! {
    PartialEq<String>:

    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_str())
    }
}

// Gnats: Ord falls short of PartialEq, in that I can only compare to Self
/* impl_for! {
    Ord:

    fn cmp(&self, other: &Self) -> Ordering {
        if Self::FIXED {
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
    <2> PartialOrd<self2!()>:

    // This is less optimised than eq, as the filler after len is greater than valid characters.
    fn partial_cmp(&self, other: &self2!()) -> Option<Ordering> {
        Some(if Self::FIXED && other.is_fixed() {
            self.str[..].cmp(&other.str[..])
        } else if Self::FIXED {
            self.str[..].cmp(other.as_bytes())
        } else if other.is_fixed() {
            self.as_bytes().cmp(&other.str[..])
        } else {
            self.as_bytes().cmp(other.as_bytes())
        })
    }
}

impl_for! {
    <'a, 2> PartialOrd<&'a self2!()>:

    fn partial_cmp(&self, other: &&'a self2!()) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl_for! {
    PartialOrd<str>:

    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        if Self::FIXED {
            self.str[..].partial_cmp(other.as_bytes())
        } else {
            self.as_bytes().partial_cmp(other.as_bytes())
        }
    }
}

impl_for! {
    <'a> PartialOrd<&'a str>:

    #[inline(always)]
    fn partial_cmp(&self, other: &&'a str) -> Option<Ordering> {
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

    // Compare each with self and every other
    macro_rules! cmp_all {
        ($op:tt) => {
            cmp_all!($op:
                stringlet!(""),
                stringlet!(v: ""),
                stringlet!(v 1: ""),
                stringlet!(v 2: ""),
                stringlet!(t: ""),
                stringlet!(t 1: ""),
                stringlet!(s: ""),
                stringlet!(s 1: ""),
                stringlet!(s 2: ""),
                stringlet!("x"),
                stringlet!(v: "x"),
                stringlet!(v 2: "x"),
                stringlet!(v 3: "x"),
                stringlet!(t: "x"),
                stringlet!(t 2: "x"),
                stringlet!(s: "x"),
                stringlet!(s 2: "x"),
                stringlet!(s 3: "x"),
                stringlet!("y"),
                stringlet!(v: "y"),
                stringlet!(v 2: "y"),
                stringlet!(v 3: "y"),
                stringlet!(t: "y"),
                stringlet!(t 2: "y"),
                stringlet!(s: "y"),
                stringlet!(s 2: "y"),
                stringlet!(s 3: "y"),
                stringlet!("xy"),
                stringlet!(v: "xy"),
                stringlet!(v 3: "xy"),
                stringlet!(v 4: "xy"),
                stringlet!(t: "xy"),
                stringlet!(t 3: "xy"),
                stringlet!(s: "xy"),
                stringlet!(s 3: "xy"),
                stringlet!(s 4: "xy"),
                /* These do not really improve coverage, but explode combinatorics:
                stringlet!("xyz"),
                stringlet!(v: "xyz"),
                stringlet!(v 4: "xyz"),
                stringlet!(v 5: "xyz"),
                stringlet!(t: "xyz"),
                stringlet!(t 4: "xyz"),
                stringlet!(s: "xyz"),
                stringlet!(s 4: "xyz"),
                stringlet!(s 5: "xyz"), */
            );
        };
        ($op:tt: $a:expr, $($rest:expr,)+) => {
            let a = $a;
            assert_eq!(a $op a.clone(), a.as_str() $op a.as_str(), "{a:#?}");
            //assert_eq!(a.as_str() $op a, a.as_str() $op a.as_str(), "{a:#?}");
            assert_eq!(a $op a.as_str(), a.as_str() $op a.as_str(), "{a:#?}");
            let ac = const { $a };
            assert_eq!(a $op ac, a.as_str() $op ac.as_str(), "{a:#?} {ac:#?}");
            $(
                let b = $rest;
                assert_eq!(a $op b, a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
                //assert_eq!(a.as_str() $op b, a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
                assert_eq!(a $op b.as_str(), a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
                assert_eq!(b $op a, b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
                //assert_eq!(b.as_str() $op a, b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
                assert_eq!(b $op a.as_str(), b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
            )+
            cmp_all!($op: $($rest,)+);
        };
        ($op:tt: $a:expr,) => {};
    }

    #[test]
    fn test_eq() {
        // Compare all kinds with enough variation in len and SIZE
        cmp_all!(==);
    }

    #[test]
    fn test_lt() {
        // Compare all kinds with enough variation in len and SIZE
        cmp_all!(<);
    }

    #[test]
    fn test_le() {
        // Compare all kinds with enough variation in len and SIZE
        //cmp_all!(<=);
    }
}
