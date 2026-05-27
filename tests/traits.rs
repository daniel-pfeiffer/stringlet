//! Test functionality of the `traits` module.

use stringlet::prelude::*;

#[test]
fn from_string() {
    let s: SlimStringlet<4> = String::from("hey").try_into().unwrap();
    assert_eq!(s.as_ref(), "hey");
}

#[test]
fn from_long_str() {
    let s: VarStringlet<16> = "Rustacean".try_into().unwrap();
    assert_eq!(&s, "Rustacean");
}

#[test]
#[should_panic]
fn panics_when_too_long() {
    let _s: VarStringlet<2> = "hello world".try_into().unwrap();
}

#[test]
fn try_from() {
    let s = SlimStringlet::<8>::try_from("hello").unwrap();
    assert_eq!(s.as_ref(), "hello");
}

#[test]
fn from_str() {
    let s = <SlimStringlet<8> as std::str::FromStr>::from_str("hello").unwrap();
    assert_eq!(s.as_ref(), "hello");
}

#[test]
fn hasher() {
    let slet = SlimStringlet::<8>::try_from("hello").unwrap();
    let hm = std::collections::HashSet::from([slet]);
    assert!(hm.contains(&slet));
}
