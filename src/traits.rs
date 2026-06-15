//! Many implementations to make stringlet easy to use.

use crate::*;

use core::hash::{Hash, Hasher};

impl_for! {
    Hash:

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
    }
}
