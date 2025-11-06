//! `stringlet!()`

#[doc(hidden)]
#[macro_export]
macro_rules! stringlet_inner {
    (1 @ $capacity:tt, $fixed:literal, $str:expr) => {
        $crate::Stringlet::<$capacity, $fixed>::_from_macro($str)
    };
    (2 @ $capacity:tt, $fixed:literal, $str:expr) => {
        const { $crate::stringlet_inner!(1 @ $capacity, $fixed, $str) }
    };
    (3 @ $capacity:tt, $fixed:literal, [$($str:expr),*$(,)?]) => {
        const { [ $(
            $crate::stringlet_inner!(1 @ $capacity, $fixed, $str)
        ),* ] }
    };
    () => {}
}

/**
Turn a const `str` expression into the smallest `Stringlet` that can contain it.
Shorthand to optionally give generic parameters `CAPACITY` and `FIXED`. For now,
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
    // stringlet!(=1: ["a", "b, "c"]) or stringlet!(=1: "a")
    (=$capacity:tt: [$($str:tt)*]) => {
        $crate::stringlet_inner!(3 @ $capacity, true, [$($str)*])
    };
    (=$capacity:tt: $str:expr) => {
        $crate::stringlet_inner!(2 @ $capacity, true, $str)
    };

    // stringlet!(= ["a", "b, "c"]) or stringlet!(= "a")
    (=$(:)? [$str:expr $(, $($strn:tt)*)?]) => {
        // As $str must be const, the optimizer should evaluate it only once…
        $crate::stringlet_inner!(2 @ { ($str).len() }, true, [$str $($strn)*])
    };
    (=$(:)? $str:expr) => {
        $crate::stringlet_inner!(2 @ { ($str).len() }, true, $str)
    };

    // stringlet!(1: ["a", "b, "c"]) or stringlet!(1: "a")
    ($capacity:tt: [$($str:tt)*]) => {
        $crate::stringlet_inner!(3 @ $capacity, false, [$($str)*])
    };
    ($capacity:tt: $str:expr) => {
        $crate::stringlet_inner!(2 @ $capacity, false, $str)
    };

    // stringlet!(["a", "b, "c"]) or stringlet!("a")
    ([$str:expr $(, $($strn:tt)*)?]) => {
        $crate::stringlet_inner!(2 @ { ($str).len() }, false, [$str $($strn)*])
    };
    ($str:expr) => {
        $crate::stringlet_inner!(2 @ { ($str).len() }, false, $str)
    };
}
