//! `Eq` & `Ord` implementations

use crate::*;

use core::cmp::Ordering;

/// A 2<sup>nd</sup> generic `StringletBase`.
macro_rules! self2 {
    () => {
        StringletBase<Kind2, SIZE2>
    };
}

// Needed where explicitly requested, e.g. HashMap key
impl_for! { Eq }

impl_for! {
    <2> PartialEq<self2!()>:

    #[inline]
    fn eq(&self, other: &self2!()) -> bool {
        /// Workaround for neither being able to `type Self2 = …<…>` nor to make this expr a const.
        macro_rules! low {
            ($t:ty, $size:ident) => {
                $size.saturating_sub(if <$t>::TRIM { 1 } else if <$t>::SLIM { 64 } else { 256 })
            };
        }

        if SIZE == 0 {
            other.is_empty()
        } else if SIZE2 == 0 {
            self.is_empty()
        } else if SIZE == SIZE2 && Kind::VAR == Kind2::VAR {
            if Kind::VAR {
                // Compare raw bytes, including the padding and len byte.
                self.as_slice() == other.as_slice()
            } else {
                // TRIM’s and SLIM’s padding make this valid, also comparing with FIXED.
                self.str == other.str[..]
            }
        } else if Kind::FIXED {
            // Size differs, so both fixed can’t be same. Can only be same if SIZE falls within other’s range.
            !Kind2::FIXED &&
                low!(Kind2, SIZE2) <= SIZE && SIZE <= SIZE2 &&
                self.str == other.as_bytes()
        } else if Kind2::FIXED {
            // Can only be same if other’s SIZE falls within self’s range.
            low!(Kind, SIZE) <= SIZE2 && SIZE2 <= SIZE &&
                other.str == self.as_bytes()
        } else {
            // Can only be same if both len()..SIZE ranges overlap.
            low!(Kind2, SIZE2) <= SIZE && low!(Kind, SIZE) <= SIZE2 &&
                self.as_bytes() == other.as_bytes()
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
        } else if Kind::FIXED {
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

impl_for! {
    <2> PartialOrd<self2!()>:

    // This is less optimised than eq, as the filler after len can’t be less than valid characters.
    fn partial_cmp(&self, other: &self2!()) -> Option<Ordering> {
        Some(if SIZE == 0 {
            if other.is_empty() { Ordering::Equal } else { Ordering::Less }
        } else if SIZE2 == 0 {
            if self.is_empty() { Ordering::Equal } else { Ordering::Greater }
        } else if Kind::FIXED {
            self.str[..]
                .cmp(if Kind2::FIXED { &other.str[..] } else { other.as_bytes() })
        } else if Kind2::FIXED {
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
        if SIZE == 0 {
            Some(if other.is_empty() { Ordering::Equal } else { Ordering::Less })
        } else {
            if Kind::FIXED { &self.str[..] } else { self.as_bytes() }
                .partial_cmp(other.as_bytes())
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
                stringlet!(v 1: "\0"),
                stringlet!(v 2: "x"),
                stringlet!(v 3: "x"),
                stringlet!(t: "x"),
                stringlet!(t 2: "x"),
                stringlet!(t 2: "\0"),
                stringlet!(s: "x"),
                stringlet!(s 2: "x"),
                stringlet!(s 2: "\0"),
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
