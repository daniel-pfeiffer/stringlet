#![doc = include_str!("../README.md")]

// todo more efficient starts_with, contains, combined Length&Utf8Error instead of panic
mod fmt;
mod impl_for;
mod macros;
mod methods;
mod new;
mod refs;

/// Configure `Stringlet` to have only valid `SIZE` and `ALIGN`.
#[diagnostic::on_unimplemented(
    message = "`Stringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=64`",
    note = "`Stringlet` cannot be longer than 64 bytes. Consider using `String`!",
    note = "Also ALIGN is {ALIGN}. This must be one of 1, 2, 4, 8, 16, 32, or 64!"
)]
pub trait Config<const SIZE: usize, const ALIGN: u8 = 1> {
    type Aligned: Copy + Eq;
}

macro_rules! config {
    ($align:literal, $aligned:ident @ $($size:literal),+) => {
        $(impl<const FIXED: bool> Config<$size, $align> for Stringlet<$size, FIXED, $align> { type Aligned = $aligned; })+
    };
    ($($aligned:ident, $fixed_stringlet:ident $(, $stringlet:ident)? => $align:literal;)+) => {
        $(
            #[doc(hidden)]
            #[repr(align($align))]
            #[derive(Copy, Clone, PartialEq, Eq)]
            pub struct $aligned(());
            config! {
                $align, $aligned @
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
                49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
            }
        )+
        $(
            $(
                #[doc = concat!("Variable length Stringlet, aligned to ", $align, " bytes")]
                pub type $stringlet<const SIZE: usize = 16> = Stringlet<SIZE, false, $align>;
            )?
            #[doc = concat!("Fixed length Stringlet, aligned to ", $align, " bytes")]
            pub type $fixed_stringlet<const SIZE: usize = 16> = Stringlet<SIZE, true, $align>;
        )+
    };
}
config! {
    Align1, FixedStringlet => 1;
    Align2, FixedStringlet2, Stringlet2 => 2;
    Align4, FixedStringlet4, Stringlet4 => 4;
    Align8, FixedStringlet8, Stringlet8 => 8;
    Align16, FixedStringlet16, Stringlet16 => 16;
    Align32, FixedStringlet32, Stringlet32 => 32;
    Align64, FixedStringlet64, Stringlet64 => 64;
}

/// An inline String 0 to 64 bytes, which can be handled like a primitive type.
#[derive(Copy, Clone, Eq)]
pub union Stringlet<const SIZE: usize = 16, const FIXED: bool = false, const ALIGN: u8 = 1>
where
    Self: Config<SIZE, ALIGN>,
{
    /// Zero size type that is aligned according to `ALIGN` and never touched.
    _align: <Self as Config<SIZE, ALIGN>>::Aligned,
    /// The actual payload, if it is shorter than SIZE, its last bytes will be tagged.
    pub(crate) str: [u8; SIZE],
}

/// Safely get out what is in the inner circle of the Stringlet wrapper.
macro_rules! o {
    ($str:ident) => {
        // SAFETY: We never touch the other union member and all bytes are guaranteed to to be initialized.
        unsafe { $str.str }
    };
}
pub(crate) use o;
