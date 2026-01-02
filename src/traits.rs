//! Many implementations to make stringlet easy to use.

use crate::*;

use core::{
    convert::Infallible,
    hash::{Hash, Hasher},
    str::FromStr,
};

impl_for! {
    bound From<String>:

    fn from(str: String) -> Self {
        Self::from(str.as_str())
    }
}

impl_for! {
    bound From<&str>:

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

impl_for! {
    bound FromStr:

    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl_for! {
    Hash:

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
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
}
