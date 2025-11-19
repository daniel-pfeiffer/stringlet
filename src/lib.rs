#![doc = include_str!("../README.md")]

mod cmp;
mod fmt;
mod macros;
mod methods;
mod new;
mod refs;
mod traits;

/// Configure `StringletBase` to have only valid generic parameters.
#[diagnostic::on_unimplemented(
    message = "`VarStringlet<{SIZE}>` or `SlimStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=255` or `0..=64`",
    note = "`VarStringlet` cannot be longer than 255 bytes. Consider using `String`!",
    note = "`SlimStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!"
)]
#[doc(hidden)]
pub trait Config<
    const SIZE: usize,
    // Might combine these in an enum, but they are not allowed for generic parameters:
    const FIXED: bool = true,
    const LEN: usize = 0,
    const ALIGN: u8 = 1,
>
{
    type Aligned: Copy + Eq + Ord;
}

macro_rules! config {
    ($msg:literal: $stringlet:ident, $aligned:ident, $fixed:tt, $len:literal, 1) => {
        #[doc = concat!($msg, " length Stringlet")]
        pub type $stringlet<const SIZE: usize = 16> =
            StringletBase<SIZE, $fixed, $len, 1>;
    };
    ($msg:literal: $stringlet:ident, $aligned:ident, $fixed:tt, $len:literal, $align:literal) => {
        #[doc = concat!($msg, " length Stringlet", ", aligned to ", $align, " bytes")]
        pub type $stringlet<const SIZE: usize = 16> =
            StringletBase<SIZE, $fixed, $len, $align>;
    };
    ($($stringlet:ident, $var_stringlet:ident, $slim_stringlet:ident: $aligned:ident @ $align:literal;)+) => {
        $(
            #[doc(hidden)]
            #[repr(align($align))]
            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $aligned;
        )+
        $(
            config!("Fixed": $stringlet, $aligned, true, 0, $align);
            config!("Variable": $var_stringlet, $aligned, false, 1, $align);
            config!("Slim variable": $slim_stringlet, $aligned, false, 0, $align);

            // todo Is there an easier way to configure all valid sizes?
            config![ // for VarStringlet and SlimStringlet
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64: $aligned @ $align
            ];
            config![ // for VarStringlet only
                + 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
                88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
                109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
                128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146,
                147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165,
                166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
                185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203,
                204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222,
                223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241,
                242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255: $aligned @ $align
            ];
        )+
    };
    ($($size:literal),+: $aligned:ident @ $align:literal) => {
        impl<const SIZE: usize> Config<SIZE, true, 0, $align>
            for StringletBase<SIZE, true, 0, $align> { type Aligned = $aligned; }
        config![+ $($size),+: $aligned @ $align];
        $(
            impl Config<$size, false, 0, $align>
                for StringletBase<$size, false, 0, $align> { type Aligned = $aligned; }
        )+
    };
    (+ $($size:literal),+: $aligned:ident @ $align:literal) => {
        $(
            impl Config<$size, false, 1, $align>
                for StringletBase<$size, false, 1, $align> { type Aligned = $aligned; }
        )+
    };
}

config! {
    Stringlet,   VarStringlet,   SlimStringlet:   Align1  @  1;
    Stringlet2,  VarStringlet2,  SlimStringlet2:  Align2  @  2;
    Stringlet4,  VarStringlet4,  SlimStringlet4:  Align4  @  4;
    Stringlet8,  VarStringlet8,  SlimStringlet8:  Align8  @  8;
    Stringlet16, VarStringlet16, SlimStringlet16: Align16 @ 16;
    Stringlet32, VarStringlet32, SlimStringlet32: Align32 @ 32;
    Stringlet64, VarStringlet64, SlimStringlet64: Align64 @ 64;
}

/// An inline String 0 to 64 bytes, which can be handled like a primitive type.
#[derive(Copy, Clone)]
pub struct StringletBase<
    const SIZE: usize,
    const FIXED: bool = true,
    const LEN: usize = 0,
    const ALIGN: u8 = 1,
> where
    Self: Config<SIZE, FIXED, LEN, ALIGN>,
{
    /// Zero size type that is aligned according to `ALIGN` and never touched.
    pub(crate) _align: [<Self as Config<SIZE, FIXED, LEN, ALIGN>>::Aligned; 0],
    /// The actual payload, if it is shorter than SIZE, either LEN == 1 or its last bytes will be tagged.
    pub(crate) str: [u8; SIZE],
    /// Limited by `Config<SIZE, FIXED, LEN, ALIGN>` to either 0 or 1 byte for an explicit length.
    // Can’t make previous SIZE + LEN, as “generic parameters may not be used in const operations”
    pub(crate) len: [u8; LEN],
}

/** Hide repeated boilerplate caused by lack of nested impls.
```ignore
impl_for! { SomeTrait }
impl_for! { <'a, 2> SomeTrait: impl_body }
```
where `'a`, `2` and impl_body are optional, `2` meaning 2<sup>nd</sup> Stringlet.
*/
macro_rules! impl_for {
    ($two:literal [$($lt:lifetime)?] $trait:ty $(: $($rest:tt)+)?) => {
        impl<
            $($lt,)? const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8,
            const SIZE2: usize, const FIXED2: bool, const LEN2: usize, const ALIGN2: u8,
        > $trait
            for StringletBase<SIZE, FIXED, LEN, ALIGN>
        where
            Self: Config<SIZE, FIXED, LEN, ALIGN>,
            StringletBase<SIZE2, FIXED2, LEN2, ALIGN2>: Config<SIZE2, FIXED2, LEN2, ALIGN2>,
        {
             $($($rest)+)?
        }
    };
    ([$($lt:lifetime)?] $trait:ty $(: $($rest:tt)+)?) => {
        impl<
            $($lt,)? const SIZE: usize, const FIXED: bool, const LEN: usize, const ALIGN: u8,
        > $trait
            for StringletBase<SIZE, FIXED, LEN, ALIGN>
        where
            Self: Config<SIZE, FIXED, LEN, ALIGN>,
        {
             $($($rest)+)?
        }
    };
    // Split this, as compiler says optional <…> is ambiguous
    (<$($lt:lifetime)? $(,)? $($two:literal)?> $trait:ty $(: $($rest:tt)+)?) => {
        impl_for!($($two)? [$($lt)?] $trait $(: $($rest)+)?);
    };
    ($trait:ty $(: $($rest:tt)+)?) => {
        impl_for!([] $trait $(: $($rest)+)?);
    };
}

pub(crate) use impl_for;
