//! Test functionality of the `cmp` module.

use stringlet::prelude::*;

/// Compare all kinds and various sizes with self and every other
macro_rules! cmp_all {
    ($fn:ident: $op:tt) => {
        #[test]
        fn $fn() {
            cmp_all!($op:
                stringlet!(""),
                stringlet!(v: ""),
                stringlet!(v 1: ""),
                stringlet!(v 2: ""),
                stringlet!(t: ""),
                stringlet!(t 1: ""),
                stringlet!(s: ""),
                stringlet!(s 1: ""),
                stringlet!(s 2: ""),
                stringlet!("x"),
                stringlet!(v: "x"),
                stringlet!(v 1: "\0"),
                stringlet!(v 2: "x"),
                stringlet!(v 3: "x"),
                stringlet!(t: "x"),
                stringlet!(t 2: "x"),
                stringlet!(t 2: "\0"),
                stringlet!(s: "x"),
                stringlet!(s 2: "x"),
                stringlet!(s 2: "\0"),
                stringlet!(s 3: "x"),
                stringlet!("y"),
                stringlet!(v: "y"),
                stringlet!(v 2: "y"),
                stringlet!(v 3: "y"),
                stringlet!(t: "y"),
                stringlet!(t 2: "y"),
                stringlet!(s: "y"),
                stringlet!(s 2: "y"),
                stringlet!(s 3: "y"),
                stringlet!("xy"),
                stringlet!(v: "xy"),
                stringlet!(v 3: "xy"),
                //stringlet!(v 4: "xy"),
                stringlet!(t: "xy"),
                stringlet!(t 3: "xy"),
                stringlet!(s: "xy"),
                stringlet!(s 3: "xy"),
                /* These do not really improve coverage, but explode combinatorics:
                stringlet!(s 4: "xy"),
                stringlet!("xyz"),
                stringlet!(v: "xyz"),
                stringlet!(v 4: "xyz"),
                stringlet!(v 5: "xyz"),
                stringlet!(t: "xyz"),
                stringlet!(t 4: "xyz"),
                stringlet!(s: "xyz"),
                stringlet!(s 4: "xyz"),
                stringlet!(s 5: "xyz"), */
            );
        }
    };
    ($op:tt: $a:expr, $($rest:expr,)+) => {
        let a = $a;
        assert_eq!(a $op a.clone(), a.as_str() $op a.as_str(), "{a:#?}");
        //assert_eq!(a.as_str() $op a, a.as_str() $op a.as_str(), "{a:#?}");
        assert_eq!(a $op a.as_str(), a.as_str() $op a.as_str(), "{a:#?}");
        let ac = const { $a };
        assert_eq!(a $op ac, a.as_str() $op ac.as_str(), "{a:#?} {ac:#?}");
        $(
            let b = $rest;
            assert_eq!(a $op b, a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
            //assert_eq!(a.as_str() $op b, a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
            assert_eq!(a $op b.as_str(), a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
            assert_eq!(b $op a, b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
            //assert_eq!(b.as_str() $op a, b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
            assert_eq!(b $op a.as_str(), b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
        )+
        cmp_all!($op: $($rest,)+);
    };
    ($op:tt: $a:expr,) => {};
}

cmp_all!(eq: ==);
cmp_all!(lt: <);
cmp_all!(le: <=);
cmp_all!(gt: >);
cmp_all!(ge: >=);

#[test]
fn cmp_wrappers() {
    let slet = stringlet!("wow");
    let string = String::from("wow");
    assert!(slet == "wow");
    assert!(slet >= "wow");

    assert!(slet == &slet);
    assert!(slet >= &slet);
    assert!(slet.cmp(&slet) == std::cmp::Ordering::Equal);

    assert!(slet == string);
    assert!(slet >= string);

    assert!(slet == &string);
    assert!(slet >= &string);
}
