//! Many implementations to make Stringlet easy to use.

use super::Stringlet;
use crate::repr::*;

use core::{
    hash::{Hash, Hasher},
    ops::Deref,
};

impl<const CAPACITY: usize, const FIXED: bool> From<String> for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    fn from(str: String) -> Self {
        Self::from(str.as_str())
    }
}

impl<const CAPACITY: usize, const FIXED: bool> From<&str> for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    fn from(str: &str) -> Self {
        assert!(
            str.len() <= CAPACITY,
            "{}::from(): cannot store {} characters",
            std::any::type_name::<Self>(),
            str.len()
        );
        // SAFETY we checked the length and str is UTF-8
        unsafe { Self::from_utf8_unchecked(str.as_bytes()) }
    }
}

impl<const CAPACITY: usize, const FIXED: bool> std::str::FromStr for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<const CAPACITY: usize, const FIXED: bool> Default for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize, const FIXED: bool> Deref for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const CAPACITY: usize, const FIXED: bool> Hash for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.repr.hash(state);
        }
    }
}

impl<const CAPACITY: usize, const FIXED: bool> PartialEq for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    fn eq(&self, other: &Self) -> bool {
        // SAFETY: all bytes are guaranteed to to be initialized
        unsafe { self.repr == other.repr }
    }
}

impl<const CAPACITY: usize, const FIXED: bool> PartialEq<str> for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<const CAPACITY: usize, const FIXED: bool> AsRef<str> for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
