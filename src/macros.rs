//! `stringlet!()`

#[doc(hidden)]
#[macro_export]
macro_rules! stringlet_inner {
    (1 @ $size:tt, $fixed:tt, $str:expr) => {
        $crate::Stringlet::<$size, $fixed>::_from_macro($str)
    };
    (2 @ $size:tt, $fixed:tt, $str:expr) => {
        const { $crate::stringlet_inner!(1 @ $size, $fixed, $str) }
    };
    (3 @ $size:tt, $fixed:tt, [$($str:expr),*$(,)?]) => {
        const { [ $(
            $crate::stringlet_inner!(1 @ $size, $fixed, $str)
        ),* ] }
    };
    () => {}
}

/**
Turn a const `str` expression into the smallest `Stringlet` that can contain it.
Shorthand to optionally give generic parameters `SIZE` and `FIXED`. For now,
please check `README.md`.

These are equivalent:
```
# use crate::stringlet::{Stringlet, stringlet};
let s1 = stringlet!("abc");
let s2: Stringlet<3> = stringlet!("abc");
let s3 = stringlet!(" abc ".trim_ascii());
assert_eq!(s1, s2);
assert_eq!(s2, s3);
```
As are these:
```
# use crate::stringlet::{Stringlet, stringlet};
let s1 = stringlet!("abcdefghijklmno");
let s2: Stringlet<15> = stringlet!("abcdefghijklmno");
let s3 = stringlet!(concat!("abcdefgh", 'i', "jklmno"));
assert_eq!(s1, s2);
assert_eq!(s2, s3);
```
Panics if the expresion is longer than 16 bytes.
```compile_fail
# use crate::stringlet::stringlet;
stringlet!("0123456789_123456789_123456789_123456789_123456789_123456789_1234"); // 65 is too long
```
*/
#[macro_export]
macro_rules! stringlet {
    // stringlet!(_: ["a", "b, "c"]) or stringlet!(_: "a")
    (_: [$($str:tt)*]) => {
        $crate::stringlet_inner!(3 @ _, _, [$($str)*])
    };
    (_: $str:expr) => {
        $crate::stringlet_inner!(2 @ _, _, $str)
    };

    // stringlet!(= ["a", "b, "c"]) or stringlet!(= "a")
    (=$(:)? [$str:expr $(, $($strn:tt)*)?]) => {
        // As $str must be const, the optimizer should evaluate it only once…
        $crate::stringlet_inner!(3 @ { ($str).len() }, true, [$str $(, $($strn)*)?])
    };
    (=$(:)? $str:expr) => {
        $crate::stringlet_inner!(2 @ { ($str).len() }, true, $str)
    };

    // stringlet!(1: ["a", "b, "c"]) or stringlet!(1: "a")
    ($size:tt: [$($str:tt)*]) => {
        $crate::stringlet_inner!(3 @ $size, false, [$($str)*])
    };
    ($size:tt: $str:expr) => {
        $crate::stringlet_inner!(2 @ $size, false, $str)
    };

    // stringlet!(["a", "b, "c"]) or stringlet!("a")
    ([$str:expr $(, $($strn:tt)*)?]) => {
        $crate::stringlet_inner!(3 @ { ($str).len() }, false, [$str $(, $($strn)*)?])
    };
    ($str:expr) => {
        $crate::stringlet_inner!(2 @ { ($str).len() }, false, $str)
    };
}
