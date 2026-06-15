//! Test functionality of the `traits` module.

use stringlet::prelude::*;

#[test]
fn hasher() {
    let slet = SlimStringlet::<8>::try_from("hello").unwrap();
    let hm = std::collections::HashSet::from([slet]);
    assert!(hm.contains(&slet));
}
