//! `stringlet!()`

#[doc(hidden)]
#[macro_export]
macro_rules! stringlet_base {
    /* (param$x:tt {: $($rest:tt)+}) => {
        stringify!($($rest)+)
    }; */
    // get size and align parameters, if not yet present
    (param$params:tt  :  $($rest:tt)+) => {
        $crate::stringlet_base!(align$params  $($rest)+)
    };
    (param($kind:tt ! $len:tt $($align:tt)?)  $size:tt:  $($rest:tt)+) => {
        $crate::stringlet_base!(align($kind $size $len $($align)?)  $($rest)+)
    };
    (param($kind:tt ! $len:tt)  $size:tt $(@ $align:tt)?:  $($rest:tt)+) => {
        $crate::stringlet_base!(align($kind $size $len $($align)?)  $($rest)+)
    };
    (param($kind:tt $size:tt $len:tt)  @ $align:tt:  $($rest:tt)+) => {
        $crate::stringlet_base!(size($kind $size $len $align)  $($rest)+)
    };

    // add default align?
    (align($kind:tt $size:tt $len:tt)  $($rest:tt)+) => {
        $crate::stringlet_base!(size($kind $size $len 1)  $($rest)+)
    };
    (align$params:tt   $($rest:tt)+) => {
        $crate::stringlet_base!(size$params  $($rest)+)
    };
    (align($kind:tt $size:tt $len:tt $align:tt)   $($rest:tt)+) => {
        $crate::stringlet_base!(size($kind $size $len $align)  $($rest)+)
    };

    // add default size?
    (size($kind:tt ! $len:tt $align:tt)  [$str:expr $(, $strn:expr)*]) => {
        $crate::stringlet_base!(gen($kind { ($str).len() } $len $align)  [$str $(, $strn)*])
    };
    (size($kind:tt ! $len:tt $align:tt)  $str:expr) => {
        $crate::stringlet_base!(gen($kind { ($str).len() } $len $align)  $str)
    };
    (size$params:tt   $($rest:tt)+) => {
        $crate::stringlet_base!(gen$params  $($rest)+)
    };

    (gen($kind:tt $size:tt $len:tt $align:tt)  [$($str:expr),+]) => {{
        use $crate::*;
        [$(
            StringletBase::<$kind, $size, $len, $align>::_from_macro($str)
        ),+]
    }};
    (gen($kind:tt $size:tt $len:tt $align:tt)  $str:expr) => {{
        use $crate::*;
        StringletBase::<$kind, $size, $len, $align>::_from_macro($str)
    }};
}

/**
Turn a const `str` expression into the smallest `Stringlet` that can contain it.
Shorthand to optionally give generic parameters `SIZE` and `Self::FIXED`. For now,
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
*/
#[macro_export]
macro_rules! stringlet {
    (_:  $($rest:tt)+) => {
        $crate::stringlet_base!(gen(_ _ _ _)  $($rest)+)
    };

    (trim  $($rest:tt)+) => {
        $crate::stringlet_base!(param(Trim ! 0)  $($rest)+)
    };
    (t  $($rest:tt)+) => {
        $crate::stringlet!(trim  $($rest)+)
    };

    (var  $($rest:tt)+) => {
        $crate::stringlet_base!(param(Var ! 1)  $($rest)+)
    };
    (v  $($rest:tt)+) => {
        $crate::stringlet!(var  $($rest)+)
    };

    (slim  $($rest:tt)+) => {
        $crate::stringlet_base!(param(Slim ! 0)  $($rest)+)
    };
    (s  $($rest:tt)+) => {
        $crate::stringlet!(slim  $($rest)+)
    };

    ($size:tt $(@ $align:tt)?: $($rest:tt)+) => {
        $crate::stringlet_base!(align(Fixed $size 0 $($align)?)  $($rest)+)
    };
    (@ $align:tt: $($rest:tt)+) => {
        $crate::stringlet_base!(size(Fixed ! 0 $align)  $($rest)+)
    };
    ($($rest:tt)+) => {
        $crate::stringlet_base!(size(Fixed ! 0 1)  $($rest)+)
    };
}

#[cfg(doctest)]
mod doctests {
    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        stringlet!(var: "0123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_12345");
    ```
    */
    fn test_macro_var_256_compile_fail() {} // VarStringlet<256> is too long

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        stringlet!(slim: "0123456789_123456789_123456789_123456789_123456789_123456789_1234"); // 65 is too long
    ```
    */
    fn test_macro_slim_65_compile_fail() {} // SlimStringlet<65> is too long
}

#[cfg(test)]
mod tests {
    use crate::*;

    /* fn cmp<const SIZE: usize, const Self::FIXED: bool, const LEN: usize, const ALIGN: u8>(slet: StringletBase<SIZE, Self::FIXED, LEN, ALIGN>, str: &str)
    where
        StringletBase<SIZE, Self::FIXED, LEN, ALIGN>: Config<SIZE, Self::FIXED, LEN, ALIGN>, */
    fn cmp<Slet: std::fmt::Debug>(slet: Slet, str: &str) {
        assert_eq!(format!("{:?}", slet), str);
    }
    #[test]
    fn test_all_types() {
        cmp(stringlet!("aha"), "Stringlet<3> { str: \"aha\" }");
        cmp(stringlet!(var: "aha"), "VarStringlet<3> { str: \"aha\" }");
        cmp(stringlet!(slim: "aha"), "SlimStringlet<3> { str: \"aha\" }");

        cmp(stringlet!(5: "aha45"), "Stringlet<5> { str: \"aha45\" }");
        cmp(stringlet!(var 5: "aha"), "VarStringlet<5> { str: \"aha\" }");
        cmp(
            stringlet!(slim 5: "aha"),
            "SlimStringlet<5> { str: \"aha\" }",
        );

        cmp(stringlet!(@2: "aha"), "Stringlet2<3> { str: \"aha\" }");
        cmp(
            stringlet!(var @2: "aha"),
            "VarStringlet2<3> { str: \"aha\" }",
        );
        cmp(
            stringlet!(slim @2: "aha"),
            "SlimStringlet2<3> { str: \"aha\" }",
        );

        cmp(
            stringlet!(5 @2: "aha45"),
            "Stringlet2<5> { str: \"aha45\" }",
        );
        cmp(
            stringlet!(var 5 @2: "aha"),
            "VarStringlet2<5> { str: \"aha\" }",
        );
        cmp(
            stringlet!(slim 5 @2: "aha"),
            "SlimStringlet2<5> { str: \"aha\" }",
        );

        let x: [Stringlet<3>; 2] = stringlet!(["aha", "oho"]);
        println!("{x:?}");
        cmp(
            stringlet!(["aha", "oho"]),
            "[Stringlet<3> { str: \"aha\" }, Stringlet<3> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(var: ["aha", "oho"]),
            "[VarStringlet<3> { str: \"aha\" }, VarStringlet<3> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(slim: ["aha", "oho"]),
            "[SlimStringlet<3> { str: \"aha\" }, SlimStringlet<3> { str: \"oho\" }]",
        );
    }
}
