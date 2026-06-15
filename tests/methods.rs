//! Test functionality of the `methods` module.

use stringlet::prelude::*;

#[test]
fn big() {
    let _f: Stringlet<1024>;
    let _v: VarStringlet<255>;
    let _s: SlimStringlet<64>;
}

#[test]
fn as_str() -> stringlet::Result<()> {
    let f: Stringlet<7> = "A123456".try_into()?;
    assert_eq!(f.as_str(), "A123456");
    let v: VarStringlet = "A123456".try_into()?;
    assert_eq!(v.as_str(), "A123456");
    let s: SlimStringlet = "A123456".try_into()?;
    assert_eq!(s.as_str(), "A123456");
    Ok(())
}

fn all_lengths<const SIZE: usize>()
where
    VarStringlet<SIZE>: stringlet::VarConfig<SIZE>,
    SlimStringlet<SIZE>: stringlet::SlimConfig<SIZE>,
{
    let str64s: [&str; 3] = [
        "0123456789_123456789_123456789_123456789_123456789_123456789_123",
        str::from_utf8(&[0; 64]).unwrap(),
        str::from_utf8(&[0x7f_u8; 64]).unwrap(),
    ];
    let fixed: Stringlet<SIZE> = (&str64s[0][..SIZE]).try_into().unwrap();
    assert_eq!(fixed.is_empty(), SIZE == 0);
    assert_eq!(fixed.len(), SIZE);
    for len in 0..=SIZE {
        let str: VarStringlet<SIZE> = (&str64s[0][..len]).try_into().unwrap();
        assert_eq!(str.is_empty(), len == 0);
        assert_eq!(str.len(), len);
        for str64 in str64s {
            if len >= const { SIZE.saturating_sub(1) } {
                let str: TrimStringlet<SIZE> = (&str64[..len]).try_into().unwrap();
                assert_eq!(str.is_empty(), len == 0);
                assert_eq!(str.len(), len);
            }
            let str: SlimStringlet<SIZE> = (&str64[..len]).try_into().unwrap();
            assert_eq!(str.is_empty(), len == 0);
            assert_eq!(str.len(), len);
        }
    }
}
#[test]
fn len() {
    macro_rules! all_lengths {
        ($($size:literal),+) => {
            $(all_lengths::<$size>();)+
        };
    }
    all_lengths![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    ];
}

#[test]
fn empty() {
    assert!(stringlet!("").is_empty());
    assert!(!stringlet!("a").is_empty());
    assert!(!stringlet!("ab").is_empty());

    assert!(stringlet!(trim: "").is_empty());
    assert!(stringlet!(trim 1: "").is_empty());
    assert!(!stringlet!(trim: "a").is_empty());
    assert!(!stringlet!(trim 2: "a").is_empty());
    assert!(!stringlet!(trim: "ab").is_empty());
    assert!(!stringlet!(trim 3: "ab").is_empty());

    assert!(stringlet!(var: "").is_empty());
    assert!(stringlet!(var 1: "").is_empty());
    assert!(stringlet!(var 2: "").is_empty());
    assert!(!stringlet!(var: "a").is_empty());
    assert!(!stringlet!(var: "ab").is_empty());

    assert!(stringlet!(slim: "").is_empty());
    assert!(stringlet!(slim 1: "").is_empty());
    assert!(stringlet!(slim 2: "").is_empty());
    assert!(!stringlet!(slim: "a").is_empty());
    assert!(!stringlet!(slim: "ab").is_empty());
}

#[test]
fn try_into() {
    let _: SlimStringlet<1> = stringlet!("").try_into().unwrap();
    let x: Result<SlimStringlet<0>, _> = stringlet!("x").try_into();
    assert!(x.is_err());
}
