//! Test functionality of the `new` module.

use stringlet::prelude::*;

#[test]
fn new() {
    let s = Stringlet::<0>::new();
    assert!(s.is_empty());
    let s = TrimStringlet::<0>::new();
    assert!(s.is_empty());
    let s = TrimStringlet::<1>::new();
    assert!(s.is_empty());
    let s = VarStringlet::<8>::new();
    assert!(s.is_empty());
    let s = SlimStringlet::<8>::new();
    assert!(s.is_empty());
}

#[test]
fn default() {
    let s: Stringlet<0> = Default::default();
    assert!(s.is_empty());
    let s: TrimStringlet<0> = Default::default();
    assert!(s.is_empty());
    let s: TrimStringlet<1> = Default::default();
    assert!(s.is_empty());
    let s: VarStringlet = Default::default();
    assert!(s.is_empty());
    let s: SlimStringlet = Default::default();
    assert!(s.is_empty());
}

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

macro_rules! from_stringlet {
    ($slet:ident ($($short:ty),+) ($($ok:ty),+) ($($long:ty),*)) => {
        $(assert_eq!(<$short>::from_stringlet($slet).unwrap_err(), TooShort, "TooShort {} {:?}", stringify!($short), $slet);)+
        $(assert_eq!(<$ok>::from_stringlet($slet).unwrap(), $slet, "Ok {} {:?}", stringify!($ok), $slet);)+
        $(assert_eq!(<$long>::from_stringlet($slet).unwrap_err(), TooLong, "TooLong {} {:?}", stringify!($long), $slet);)*
    };
    ([$($slet:tt,)+] $short:tt $ok:tt $long:tt) => {
        $(
            let slet = stringlet!$slet;
            from_stringlet!(slet $short $ok $long);
        )+
    };
}

#[test]
fn from_stringlet() {
    use stringlet::error::Error::{TooLong, TooShort};
    from_stringlet! {
        [
            (""),
            (v: ""),
            (v 1: ""),
            (v 2: ""),
            (t: ""),
            (t 1: ""),
            (s: ""),
            (s 1: ""),
            (s 2: ""),
        ]
        (Stringlet<1>, TrimStringlet<2>)
        (Stringlet<0>, VarStringlet<0>, VarStringlet<255>, TrimStringlet<0>, TrimStringlet<1>, SlimStringlet<0>, SlimStringlet<64>)
        ()
    }
    from_stringlet! {
        [
            ("x"),
            (v: "x"),
            (v 1: "\0"),
            (v 2: "x"),
            (v 3: "x"),
            (t: "x"),
            (t 2: "x"),
            (t 2: "\0"),
            (s: "x"),
            (s 2: "x"),
            (s 2: "\0"),
            (s 3: "x"),
            ("y"),
            (v: "y"),
            (v 2: "y"),
            (v 3: "y"),
            (t: "y"),
            (t 2: "y"),
            (s: "y"),
            (s 2: "y"),
            (s 3: "y"),
        ]
        (Stringlet<2>, TrimStringlet<3>)
        (Stringlet<1>, VarStringlet<1>, VarStringlet<255>, TrimStringlet<1>, TrimStringlet<2>, SlimStringlet<1>, SlimStringlet<64>)
        (Stringlet<0>, VarStringlet<0>, TrimStringlet<0>, SlimStringlet<0>)
    }
    from_stringlet! {
        [
            (65: &"x".repeat(65)),
        ]
        (Stringlet<66>, TrimStringlet<67>)
        (Stringlet<65>, VarStringlet<65>, TrimStringlet<65>)
        (Stringlet<64>, VarStringlet<64>, TrimStringlet<64>, SlimStringlet<64>)
    }
    from_stringlet! {
        [
            (256: &"x".repeat(256)),
        ]
        (Stringlet<257>, TrimStringlet<258>)
        (Stringlet<256>, TrimStringlet<256>)
        (Stringlet<255>, VarStringlet<255>, TrimStringlet<255>)
    }
}
