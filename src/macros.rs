//! `stringlet!()`

#[doc(hidden)]
#[macro_export]
macro_rules! stringlet_base {
    // get size parameter, if not yet present
    (param$params:tt  :  $($rest:tt)+) => {
        $crate::stringlet_base!(size$params  $($rest)+)
    };
    (param($kind:tt)  $size:tt:  $($rest:tt)+) => {
        $crate::stringlet_base!($crate::$kind<$size>:  $($rest)+)
    };

    // add default size?
    (size($kind:tt)  [$str:expr $(, $strn:expr)*]) => {
        $crate::stringlet_base!($crate::$kind<{ ($str).len() }>: [$str $(, $strn)*])
    };
    (size($kind:tt)  $str:expr) => {
        $crate::stringlet_base!($crate::$kind<{ ($str).len() }>:  $str)
    };
    (size($kind:tt $size:tt)  $str:expr) => {
        $crate::stringlet_base!($crate::$kind<$size>:  $str)
    };

    ($slet:ty:  [$($str:expr),+]) => {
        [$(
            <$slet>::_from_macro($str)
        ),+]
    };
    ($slet:ty:  $str:expr) => {
        <$slet>::_from_macro($str)
    };
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
        $crate::stringlet_base!($crate::StringletBase::<_, _, _>:  $($rest)+)
    };

    (trim  $($rest:tt)+) => {
        $crate::stringlet_base!(param(TrimStringlet)  $($rest)+)
    };
    (t  $($rest:tt)+) => {
        $crate::stringlet!(trim  $($rest)+)
    };

    (var  $($rest:tt)+) => {
        $crate::stringlet_base!(param(VarStringlet)  $($rest)+)
    };
    (v  $($rest:tt)+) => {
        $crate::stringlet!(var  $($rest)+)
    };

    (slim  $($rest:tt)+) => {
        $crate::stringlet_base!(param(SlimStringlet)  $($rest)+)
    };
    (s  $($rest:tt)+) => {
        $crate::stringlet!(slim  $($rest)+)
    };

    ($size:tt: $($rest:tt)+) => {
        $crate::stringlet_base!($crate::Stringlet<$size>:  $($rest)+)
    };
    ($($rest:tt)+) => {
        $crate::stringlet_base!(size(Stringlet)  $($rest)+)
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
    fn cmp<Slet: std::fmt::Debug>(slet: Slet, str: &str) {
        assert_eq!(format!("{:?}", slet), str);
    }
    #[test]
    fn test_all_types() {
        cmp(stringlet!("aha"), "Stringlet<3> { str: \"aha\" }");
        cmp(stringlet!(var: "aha"), "VarStringlet<3> { str: \"aha\" }");
        cmp(stringlet!(trim: "aha"), "TrimStringlet<3> { str: \"aha\" }");
        cmp(stringlet!(slim: "aha"), "SlimStringlet<3> { str: \"aha\" }");

        cmp(stringlet!(5: "aha45"), "Stringlet<5> { str: \"aha45\" }");
        cmp(stringlet!(var 5: "aha"), "VarStringlet<5> { str: \"aha\" }");
        cmp(
            stringlet!(trim 4: "aha"),
            "TrimStringlet<4> { str: \"aha\" }",
        );
        cmp(
            stringlet!(slim 5: "aha"),
            "SlimStringlet<5> { str: \"aha\" }",
        );

        cmp(
            stringlet!(["aha", "oho"]),
            "[Stringlet<3> { str: \"aha\" }, Stringlet<3> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(var: ["aha", "oh"]),
            "[VarStringlet<3> { str: \"aha\" }, VarStringlet<3> { str: \"oh\" }]",
        );
        cmp(
            stringlet!(trim: ["aha", "oh"]),
            "[TrimStringlet<3> { str: \"aha\" }, TrimStringlet<3> { str: \"oh\" }]",
        );
        cmp(
            stringlet!(slim: ["aha", "oh"]),
            "[SlimStringlet<3> { str: \"aha\" }, SlimStringlet<3> { str: \"oh\" }]",
        );

        cmp(
            stringlet!(3: ["aha", "oho"]),
            "[Stringlet<3> { str: \"aha\" }, Stringlet<3> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(var 5: ["aha", "oh"]),
            "[VarStringlet<5> { str: \"aha\" }, VarStringlet<5> { str: \"oh\" }]",
        );
        cmp(
            stringlet!(trim 4: ["aha", "oho"]),
            "[TrimStringlet<4> { str: \"aha\" }, TrimStringlet<4> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(slim 5: ["aha", "oh"]),
            "[SlimStringlet<5> { str: \"aha\" }, SlimStringlet<5> { str: \"oh\" }]",
        );
    }
}
