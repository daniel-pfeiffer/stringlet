//! Many implementations to make stringlet easy to use.

use crate::*;

use core::{
    hash::{Hash, Hasher},
    str::FromStr,
};

// todo error handling and TryFrom instead
impl_for! {
    From<String>:

    fn from(str: String) -> Self {
        Self::from(str.as_str())
    }
}

impl_for! {
    FromStr:

    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        //println!("from_str");
        Ok(Self::from(str))
        /*if Self::fits(str.len()) {
            // SAFETY we checked the length
            Ok(unsafe { Self::from_str_unchecked(str) })
        } else {
            Err(())
        }*/
    }
}

impl_for! {
    From<&str>:

    fn from(str: &str) -> Self {
        //println!("from");
    // todo why doesn’t this find the previous method, instead needing it to be cloned in new.rs?
        Self::from_str(str).unwrap_or_else(|_| panic!("{}::from(): cannot store {} characters",
            Self::type_name(),
            str.len()
        ))
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
