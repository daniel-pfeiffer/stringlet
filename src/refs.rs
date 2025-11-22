//! All implementations dealing with refs.

use crate::*;

use core::ops::Deref;

impl_for! {
    Deref:

    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl_for! {
    AsRef<str>:

    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

macro_rules! impl_ref {
    ($type:ty => $stringlet:ident, $kind:ident) => {
        impl<const SIZE: usize> AsRef<$stringlet<SIZE>> for $type
        where
            $stringlet<SIZE>: Config<$kind, SIZE>,
        {
            #[doc = concat!("Cast a `", stringify!($type), "` as a shared reference to a [`", stringify!($stringlet), "`].")]
            /// The size may be inferred, but it must match the input length!
            /// Alignment will be that of the input, so you can’t choose more than 1.
            #[inline]
            fn as_ref(&self) -> &$stringlet<SIZE> {
                assert_eq!(
                    self.len(),
                    SIZE,
                    concat!("Cannot cast a len {} ", stringify!($type), " as &", stringify!($stringlet), "<SIZE>"),
                    self.len()
                );
                // SAFETY I’m not sure. It seems to cast the input bytes to Stringlet just fine.
                unsafe { core::mem::transmute(&*(self.as_ptr() as *const $stringlet<SIZE>)) }
            }
        }
    };
}

impl_ref!(str => Stringlet, Fixed);
impl_ref!(str => TrimStringlet, Trim);
impl_ref!(str => SlimStringlet, Slim);

impl_ref!(String => Stringlet, Fixed);
impl_ref!(String => TrimStringlet, Trim);
impl_ref!(String => SlimStringlet, Slim);

impl_ref!(Box<str> => Stringlet, Fixed);
impl_ref!(Box<str> => TrimStringlet, Trim);
impl_ref!(Box<str> => SlimStringlet, Slim);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let s = Stringlet::<3>::from("Abc");
        assert!(s.contains('b'));
        let s = VarStringlet::<4>::from("Abc");
        assert!(s.contains('b'));
        let s = SlimStringlet::<4>::from("Abc");
        assert!(s.contains('b'));
    }

    #[test]
    fn test_as_ref() {
        let s = Stringlet::<1>::from("A");
        let s: &str = s.as_ref();
        assert_eq!(s, "A");
    }

    #[test]
    fn test_borrow() {
        macro_rules! test_borrow {
            ($a:ident = $in:expr, $size:literal) => {
                let $a = $in;
                let str: &str = $a.as_ref();
                let slet: &Stringlet<$size> = $a.as_ref();
                assert_eq!(str.as_ptr(), slet.as_ptr(), "fail {}", stringify!($in));
            };
            ($a:expr) => {
                test_borrow!(a = $a, 3);
                test_borrow!(b = &a[1..], 2);
            };
        }
        test_borrow!("aha");

        test_borrow!(String::from("aha"));

        test_borrow!(Box::<str>::from("aha"));
    }

    #[test]
    #[should_panic]
    fn test_panics_when_ref_too_long() {
        let _: &Stringlet<4> = "abc".as_ref();
    }
}
