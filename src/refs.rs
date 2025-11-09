//! All implementations dealing with refs.

use crate::*;

use core::ops::Deref;

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> Deref for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> AsRef<str>
    for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const SIZE: usize> AsRef<FixedStringlet<SIZE>> for str
where
    FixedStringlet<SIZE>: Config<SIZE, 1>,
{
    /// Cast a `str` as a shared reference to a `FixedStringlet`.
    /// The size may be inferred, but it must match the input length!
    /// Alignment will be that of the input, so you can’t choose more than 1.
    #[inline(always)]
    fn as_ref(&self) -> &FixedStringlet<SIZE> {
        assert_eq!(
            self.len(),
            SIZE,
            "Cannot cast len {} str as &FixedStringlet<{SIZE}>",
            self.len()
        );
        // SAFETY I’m not sure. It seems to cast the input bytes to Stringlet just fine.
        unsafe { core::mem::transmute(&*(self.as_ptr() as *const FixedStringlet<SIZE>)) }
    }
}

impl<const SIZE: usize> AsRef<FixedStringlet<SIZE>> for String
where
    FixedStringlet<SIZE>: Config<SIZE, 1>,
{
    /// Cast a `String` as a shared reference to a `FixedStringlet`.
    /// The size may be inferred, but it must match the input length!
    /// Alignment will be that of the input, so you can’t choose more than 1.
    #[inline(always)]
    fn as_ref(&self) -> &FixedStringlet<SIZE> {
        assert_eq!(
            self.len(),
            SIZE,
            "Cannot cast len {} String as &FixedStringlet<{SIZE}>",
            self.len()
        );
        // SAFETY I’m not sure. It seems to cast the input bytes to Stringlet just fine.
        unsafe { core::mem::transmute(&*(self.as_ptr() as *const FixedStringlet<SIZE>)) }
    }
}

impl<const SIZE: usize> AsRef<FixedStringlet<SIZE>> for Box<str>
where
    FixedStringlet<SIZE>: Config<SIZE, 1>,
{
    /// Cast a `Box<str>` as a shared reference to a `FixedStringlet`.
    /// The size may be inferred, but it must match the input length!
    /// Alignment will be that of the input, so you can’t choose more than 1.
    #[inline(always)]
    fn as_ref(&self) -> &FixedStringlet<SIZE> {
        assert_eq!(
            self.len(),
            SIZE,
            "Cannot cast len {} Box<str> as &FixedStringlet<{SIZE}>",
            self.len()
        );
        // SAFETY I’m not sure. It seems to cast the input bytes to Stringlet just fine.
        unsafe { core::mem::transmute(&*(self.as_ptr() as *const FixedStringlet<SIZE>)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let s = Stringlet::<4>::from("Abc");
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
                let fs: &FixedStringlet<$size> = $a.as_ref();
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
