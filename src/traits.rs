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
