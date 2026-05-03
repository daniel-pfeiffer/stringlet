//! `stringlet!()`

#[doc(hidden)]
#[macro_export]
macro_rules! stringlet_base {
    // get size parameter, if not yet present
    (param$params:tt  :  $($rest:tt)+) => {
        $crate::stringlet_base!(size$params  $($rest)+)
    };
    (param($kind:tt)  $size:tt:  $($rest:tt)+) => {
        $crate::stringlet_base!(size($kind $size)  $($rest)+)
    };

    // add default size or was given?
    (size($kind:tt)  [$str:expr $(, $($rest:tt)*)?]) => {{
        const STR: &::core::primitive::str = $str;
        $crate::stringlet_base!([$crate::stringlet_base!(const $kind { STR.len() } STR)] $($($rest)*)?)
    }};
    (size($kind:tt $size:tt)  [$str:literal $(, $($rest:tt)*)?]) => {
        $crate::stringlet_base!([$crate::stringlet_base!(const $kind $size $str)] $($($rest)*)?)
    };
    (size($kind:tt $size:tt)  [$str:expr $(, $($rest:tt)*)?]) => {
        $crate::stringlet_base!([$crate::stringlet_base!(dyn $kind $size $str)] $($($rest)*)?)
    };

    (size($kind:tt)  $str:expr) => {
        const {
            const STR: &::core::primitive::str = $str;
            $crate::stringlet_base!(dyn $kind { STR.len() } STR)
        }
    };
    (size($kind:tt $size:tt)  $str:literal) => {
        $crate::stringlet_base!(const $kind $size $str)
    };
    (size($kind:tt $size:tt)  $str:expr) => {
        $crate::stringlet_base!(dyn $kind $size $str)
    };

    (dyn _ _ $str:expr) => {
        $crate::StringletBase::<_, _>::_from_macro($str)
    };
    (dyn $kind:tt $size:tt $str:expr) => {
        $crate::StringletBase::<$crate::$kind, $size>::_from_macro($str)
    };

    (const $kind:tt $size:tt $str:expr) => {
        const {
            $crate::stringlet_base!(dyn $kind $size $str)
        }
    };

    ([$($done:expr),*]  $str:literal $(, $($rest:tt)*)?) => {
        $crate::stringlet_base!([$($done),*, $crate::stringlet_base!(const _ _ $str)]  $($($rest)*)?)
    };
    /* ([$($done:expr),*]  $($str:literal),+ $(, $($rest:tt)*)?) => {
        $crate::stringlet_base!([$($done),*, $($crate::stringlet_base!(const _ _ $str)),+]  $($($rest)*)?)
    }; */
    ([$($done:expr),*]  $str:expr $(, $($rest:tt)*)?) => {
        $crate::stringlet_base!([$($done),*, $crate::stringlet_base!(dyn _ _ $str)]  $($($rest)*)?)
    };
    ($array:expr) => {
        $array
    };
}

/**
Turn a `str` expression into the smallest `Stringlet` that can contain it.
Or turn `[str, …]` into an array of the smallest `Stringlet` that can contain them.
You can explicitly ask for other kinds of stringlet. By default `SIZE` is the
length of the 1st `str` parameter, in which case that parameter must be `const`.
You can also give the size explicitly, or have it inferred from context along
with the kind.

The optional configuration is kind and/or size, if present followed by a colon:

|Long Spec \| |Short Spec \| |Type |
|:---|:---|:---|
| | |`Stringlet<param.len()>`|
|SIZE: | |`Stringlet<SIZE>`|
|var: |v: |`VarStringlet<param.len()>`|
|var SIZE: |v SIZE: |`VarStringlet<SIZE>`|
|trim: |t: |`TrimStringlet<param.len()>`|
|trim SIZE: |t SIZE: |`TrimStringlet<SIZE>`|
|slim: |s: |`SlimStringlet<param.len()>`|
|slim SIZE: |s SIZE: |`SlimStringlet<SIZE>`|
|_: | |`StringletBase<_, _>`|

These are equivalent:
```
# use crate::stringlet::{Stringlet, stringlet};
let s1 = stringlet!("abc");
let s2: Stringlet<3> = stringlet!(_: "abc");
const S3: Stringlet<3> = stringlet!(3: " abc ".trim_ascii());
assert_eq!(s1, s2);
assert_eq!(s2, S3);
```
As are these:
```
# use crate::stringlet::{VarStringlet, stringlet};
let s1 = stringlet!(var: ["abcdefghijklmno", "xyz"]);
let s2: [VarStringlet<15>; 2] = stringlet!(_: [&String::from("abcdefghijklmno"), "xyz"]);
const S3: [VarStringlet<15>; 2] = stringlet!(v 15: [concat!("abcdefgh", 'i', "jklmno"), "xyz"]);
assert_eq!(s1, s2);
assert_eq!(s2, S3);
```
*/
#[macro_export]
macro_rules! stringlet {
    (_:  $($rest:tt)+) => {
        $crate::stringlet_base!(size(_ _)  $($rest)+)
    };

    (var  $($rest:tt)+) => {
        $crate::stringlet_base!(param(Var)  $($rest)+)
    };
    (v  $($rest:tt)+) => {
        $crate::stringlet!(var  $($rest)+)
    };

    (trim  $($rest:tt)+) => {
        $crate::stringlet_base!(param(Trim)  $($rest)+)
    };
    (t  $($rest:tt)+) => {
        $crate::stringlet!(trim  $($rest)+)
    };

    (slim $($rest:tt)+) => {
        $crate::stringlet_base!(param(Slim)  $($rest)+)
    };
    (s  $($rest:tt)+) => {
        $crate::stringlet!(slim  $($rest)+)
    };

    ($size:tt:  $($rest:tt)+) => {
        $crate::stringlet_base!(size(Fixed $size)  $($rest)+)
    };
    ($($rest:tt)+) => {
        $crate::stringlet_base!(size(Fixed)  $($rest)+)
    };
}

#[cfg(doctest)]
mod doctests {
    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        let _: Stringlet<2> = stringlet!(_: ""); // 0 is too short
    ```
    */
    fn test_macro_fixed_2_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        let _: [Stringlet<2>; 2] = stringlet!(_: ["ok", ""]); // 0 is too short
    ```
    */
    fn test_macro_fixed_array_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(2: ""); // 0 is too short
    ```
    */
    fn test_macro_fixed_2_0_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(2: "more"); // 4 is too long
    ```
    */
    fn test_macro_fixed_2_4_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(var: "0123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_12345");
    ```
    */
    fn test_macro_var_256_compile_fail() {} // VarStringlet<256> is too long

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(trim 2: ""); // 0 is too short
    ```
    */
    fn test_macro_trim_2_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(trim: ["ok", ""]); // 0 is too short
    ```
    */
    fn test_macro_trim_array_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(trim 2: ["ok", ""]); // 0 is too short
    ```
    */
    fn test_macro_trim_array_2_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
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
