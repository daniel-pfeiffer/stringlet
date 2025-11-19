//! All implementations dealing with refs.

use crate::*;

use core::ops::Deref;

impl_for! {
    Deref:

    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl_for! {
    AsRef<str>:

    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const SIZE: usize> AsRef<Stringlet<SIZE>> for str
where
    Stringlet<SIZE>: Config<SIZE>,
{
    /// Cast a `str` as a shared reference to a fixed size `Stringlet`.
    /// The size may be inferred, but it must match the input length!
    /// Alignment will be that of the input, so you can’t choose more than 1.
    #[inline(always)]
    fn as_ref(&self) -> &Stringlet<SIZE> {
        assert_eq!(
            self.len(),
            SIZE,
            "Cannot cast len {} str as &Stringlet<{SIZE}>",
            self.len()
        );
        // SAFETY I’m not sure. It seems to cast the input bytes to Stringlet just fine.
        unsafe { core::mem::transmute(&*(self.as_ptr() as *const Stringlet<SIZE>)) }
    }
}

impl<const SIZE: usize> AsRef<Stringlet<SIZE>> for String
where
    Stringlet<SIZE>: Config<SIZE>,
{
    /// Cast a `String` as a shared reference to a fixed size `Stringlet`.
    /// The size may be inferred, but it must match the input length!
    /// Alignment will be that of the input, so you can’t choose more than 1.
    #[inline(always)]
    fn as_ref(&self) -> &Stringlet<SIZE> {
        assert_eq!(
            self.len(),
            SIZE,
            "Cannot cast len {} String as &Stringlet<{SIZE}>",
            self.len()
        );
        // SAFETY I’m not sure. It seems to cast the input bytes to Stringlet just fine.
        unsafe { core::mem::transmute(&*(self.as_ptr() as *const Stringlet<SIZE>)) }
    }
}

impl<const SIZE: usize> AsRef<Stringlet<SIZE>> for Box<str>
where
    Stringlet<SIZE>: Config<SIZE>,
{
    /// Cast a `Box<str>` as a shared reference to a fixed size `Stringlet`.
    /// The size may be inferred, but it must match the input length!
    /// Alignment will be that of the input, so you can’t choose more than 1.
    #[inline(always)]
    fn as_ref(&self) -> &Stringlet<SIZE> {
        assert_eq!(
            self.len(),
            SIZE,
            "Cannot cast len {} Box<str> as &Stringlet<{SIZE}>",
            self.len()
        );
        // SAFETY I’m not sure. It seems to cast the input bytes to Stringlet just fine.
        unsafe { core::mem::transmute(&*(self.as_ptr() as *const Stringlet<SIZE>)) }
    }
}

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
                let fs: &Stringlet<$size> = $a.as_ref();
                assert_ne!(
                    format!("{:p}", $a),
                    format!("{:p}", fs),
                    "fail {}",
                    stringify!($in)
                );
            };
            ($a:expr) => {
                test_borrow!(a = $a, 3);
                test_borrow!(b = &a[1..], 2);
            };
        }
        test_borrow!("aha");

        test_borrow!(&String::from("aha")[..]);

        test_borrow!(Box::<str>::from("aha"));
    }
}
