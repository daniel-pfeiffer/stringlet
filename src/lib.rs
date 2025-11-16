#![doc = include_str!("../README.md")]

mod fmt;
mod impl_for;
mod macros;
mod methods;
mod new;
mod refs;

/// Configure `StringletBase` to have only valid generic parameters.
#[diagnostic::on_unimplemented(
    message = "`Stringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=64`",
    note = "`Stringlet` cannot be longer than 64 bytes. Consider using `String`!"
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
    type Aligned: Copy + Eq;
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
            #[derive(Copy, Clone, PartialEq, Eq)]
            pub struct $aligned;
        )+
        $(
            config!("Fixed": $stringlet, $aligned, true, 0, $align);
            config!("Variable": $var_stringlet, $aligned, false, 1, $align);
            config!("Slim variable": $slim_stringlet, $aligned, false, 0, $align);
            config![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64: $aligned @ $align
            ];
        )+
    };
    ($($size:literal),+: $aligned:ident @ $align:literal) => {
        $(
            impl Config<$size, true, 0, $align>
                for StringletBase<$size, true, 0, $align> { type Aligned = $aligned; }
            impl Config<$size, false, 1, $align>
                for StringletBase<$size, false, 1, $align> { type Aligned = $aligned; }
            impl Config<$size, false, 0, $align>
                for StringletBase<$size, false, 0, $align> { type Aligned = $aligned; }
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
#[derive(Copy, Clone, Eq)]
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
