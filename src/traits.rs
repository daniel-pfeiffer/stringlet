//! Many implementations to make stringlet easy to use.

use crate::*;

use core::{
    hash::{Hash, Hasher},
    str::FromStr,
};

impl_for! {
    TryFrom<String>:

    type Error = error::Error;

    fn try_from(str: String) -> Result<Self> {
        Self::from_str(str.as_str())
    }
}

impl_for! {
    FromStr:

    type Err = error::Error;

    fn from_str(str: &str) -> Result<Self> {
        Self::from_str(str)
    }
}

impl_for! {
    TryFrom<&str>:

    type Error = error::Error;

    fn try_from(str: &str) -> Result<Self> {
        Self::from_str(str)
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
        let s: SlimStringlet<4> = String::from("hey").try_into().unwrap();
        assert_eq!(s.as_ref(), "hey");
    }

    #[test]
    fn test_from_long_str() {
        let s: VarStringlet<16> = "Rustacean".try_into().unwrap();
        assert_eq!(&s, "Rustacean");
    }

    #[test]
    #[should_panic]
    fn test_panics_when_too_long() {
        let _s: VarStringlet<2> = "hello world".try_into().unwrap();
    }

    #[test]
    fn test_from_str() {
        let s = SlimStringlet::<8>::try_from("hello").unwrap();
        assert_eq!(s.as_ref(), "hello");
    }
}
