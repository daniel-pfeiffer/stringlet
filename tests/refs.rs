//! Test functionality of the `refs` module.

use stringlet::prelude::*;

#[test]
fn deref() {
    let s = Stringlet::<3>::try_from("Abc").unwrap();
    assert!(s.contains('b'));
    let s = VarStringlet::<4>::try_from("Abc").unwrap();
    assert!(s.contains('b'));
    let s = TrimStringlet::<4>::try_from("Abc").unwrap();
    assert!(s.contains('b'));
    let s = SlimStringlet::<4>::try_from("Abc").unwrap();
    assert!(s.contains('b'));
}

#[test]
fn as_ref() {
    let s = Stringlet::<1>::try_from("A").unwrap();
    let s: &str = s.as_ref();
    assert_eq!(s, "A");
}

#[test]
fn borrow() {
    macro_rules! test_borrow {
        ($a:ident = $in:expr, $size:literal) => {
            let $a = $in;
            let str: &str = $a.as_ref();
            let slet: &Stringlet<$size> = $a.as_ref();
            assert_eq!(str.as_ptr(), slet.as_ptr(), "fail {}", stringify!($in));
        };
        ($a:expr) => {
            test_borrow!(a = $a, 3);
            test_borrow!(b = &a[1..], 2);
        };
    }
    test_borrow!("aha");

    test_borrow!(String::from("aha"));

    test_borrow!(Box::<str>::from("aha"));
}

#[test]
#[should_panic]
fn panics_when_ref_too_long() {
    let _: &Stringlet<4> = "abc".as_ref();
}
