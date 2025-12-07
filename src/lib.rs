#![doc = include_str!("../README.md")]
#![deny(clippy::alloc_instead_of_core, clippy::std_instead_of_core)]

/* #![no_std]
extern crate alloc; */

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
pub trait ConfigBase<Kind, const SIZE: usize = 16, const LEN: usize = 0, const ALIGN: u8 = 1> {
    type Aligned: Copy + Eq + Ord;
}

pub trait StringletKind {
    const ABBR: u8;
}
// Emulate enum, which generic can’t handle yet. No need for trait, as they’re constrained by ConfigBase.
#[derive(Copy, Clone)]
pub enum Fixed {}
impl StringletKind for Fixed {
    const ABBR: u8 = b'F';
}
pub trait Config<const SIZE: usize = 16, const ALIGN: u8 = 1>:
    ConfigBase<Fixed, SIZE, 0, ALIGN>
{
}

#[derive(Copy, Clone)]
pub enum Var {}
impl StringletKind for Var {
    const ABBR: u8 = b'V';
}
pub trait VarConfig<const SIZE: usize = 16, const ALIGN: u8 = 1>:
    ConfigBase<Var, SIZE, 1, ALIGN>
{
}

#[derive(Copy, Clone)]
pub enum Trim {}
impl StringletKind for Trim {
    const ABBR: u8 = b'T';
}
pub trait TrimConfig<const SIZE: usize = 16, const ALIGN: u8 = 1>:
    ConfigBase<Trim, SIZE, 0, ALIGN>
{
}

#[derive(Copy, Clone)]
pub enum Slim {}
impl StringletKind for Slim {
    const ABBR: u8 = b'S';
}
pub trait SlimConfig<const SIZE: usize = 16, const ALIGN: u8 = 1>:
    ConfigBase<Slim, SIZE, 0, ALIGN>
{
}

macro_rules! config {
    ($kind:ident $kind_config:ident $msg:literal: $stringlet:ident, $aligned:ident, $len:literal, 1) => {
        // Even though this comes 1st, it later complains that these types are undefined, so do them manually above
        // #[derive(Copy, Clone)]
        // pub enum $kind {}
        // trait $kind_config<const SIZE: usize = 16, const ALIGN: u8 = 1>: ConfigBase<$kind, SIZE, $len, ALIGN> {}

        #[doc = concat!($msg, " length Stringlet")]
        pub type $stringlet<const SIZE: usize = 16> =
            StringletBase<$kind, SIZE, $len, 1>;
    };
    ($kind:ident $kind_config:ident $msg:literal: $stringlet:ident, $aligned:ident, $len:literal, $align:literal) => {
        #[doc = concat!($msg, " length Stringlet, aligned to ", $align, " bytes")]
        pub type $stringlet<const SIZE: usize = 16> =
            StringletBase<$kind, SIZE, $len, $align>;
    };
    ($($stringlet:ident, $var_stringlet:ident, $trim_stringlet:ident, $slim_stringlet:ident: $aligned:ident @ $align:literal;)+) => {
        $(
            #[doc(hidden)]
            #[repr(align($align))]
            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $aligned;
        )+
        $(
            config!(Fixed Config "Fixed": $stringlet, $aligned, 0, $align);
            config!(Var VarConfig "Variable": $var_stringlet, $aligned, 1, $align);
            config!(Trim TrimConfig "Trimmed": $trim_stringlet, $aligned, 0, $align);
            config!(Slim SlimConfig "Slim variable": $slim_stringlet, $aligned, 0, $align);

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
                242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255: Var VarConfig $aligned @ 1, $align
            ];
        )+
    };
    ($($size:tt),+: $aligned:ident @ $align:literal) => {
        // todo This is partially redundant with the 4 calls above
        config![+ SIZE: Fixed Config $aligned @ $align];
        config![+ $($size),+: Var VarConfig $aligned @ 1, $align];
        config![+ SIZE: Trim TrimConfig $aligned @ $align];
        config![+ $($size),+: Slim SlimConfig $aligned @ 0, $align];
    };
    (+ SIZE: $kind:ident $kind_config:ident $aligned:ident @ $align:literal) => {
        impl<const SIZE: usize> ConfigBase<$kind, SIZE, 0, $align>
        for StringletBase<$kind, SIZE, 0, $align>
        {
            type Aligned = $aligned;
        }

        impl<const SIZE: usize> $kind_config<SIZE, $align>
        for StringletBase<$kind, SIZE, 0, $align>
        {}
    };
    (+ $($size:tt),+: $kind:ident $kind_config:ident $aligned:ident @ $len:literal, $align:literal) => {
        $(
            impl ConfigBase<$kind, $size, $len, $align>
            for StringletBase<$kind, $size, $len, $align>
            {
                type Aligned = $aligned;
            }

            impl $kind_config<$size, $align>
            for StringletBase<$kind, $size, $len, $align>
            {}
        )+
    };
}

config! {
    Stringlet,   VarStringlet,   TrimStringlet,   SlimStringlet:   Align1  @  1;
    Stringlet2,  VarStringlet2,  TrimStringlet2,  SlimStringlet2:  Align2  @  2;
    Stringlet4,  VarStringlet4,  TrimStringlet4,  SlimStringlet4:  Align4  @  4;
    Stringlet8,  VarStringlet8,  TrimStringlet8,  SlimStringlet8:  Align8  @  8;
    Stringlet16, VarStringlet16, TrimStringlet16, SlimStringlet16: Align16 @ 16;
    Stringlet32, VarStringlet32, TrimStringlet32, SlimStringlet32: Align32 @ 32;
    Stringlet64, VarStringlet64, TrimStringlet64, SlimStringlet64: Align64 @ 64;
}

/// An inline String of varying size bounds, which can be handled like a primitive type.
#[derive(Copy, Clone)]
pub struct StringletBase<
    Kind: StringletKind,
    const SIZE: usize,
    // Have to make LEN explicit, as (unlike type Aligned) we can’t pick up a const from ConfigBase.
    // We could put the whole [u8; LEN] into an associated type, but then it would be opaque to us.
    const LEN: usize = 0,
    const ALIGN: u8 = 1,
> where
    Self: ConfigBase<Kind, SIZE, LEN, ALIGN>,
{
    /// Zero size type that is aligned according to `ALIGN` and never touched.
    pub(crate) _align: [<Self as ConfigBase<Kind, SIZE, LEN, ALIGN>>::Aligned; 0],
    /// The actual payload, if it is shorter than SIZE, either LEN == 1 or its last bytes will be tagged.
    pub(crate) str: [u8; SIZE],
    /// Limited by `ConfigBase<SIZE, Self::FIXED, LEN, ALIGN>` to either 0 or 1 byte for an explicit length.
    // str can’t be SIZE + LEN, as “generic parameters may not be used in const operations”
    pub(crate) len: [u8; LEN],
}

/// A 2<sup>nd</sup> generic StringletBase
macro_rules! self2 {
    () => {
        StringletBase<Kind2, SIZE2, LEN2, ALIGN2>
    };
}

/** Impl `SomeTrait` for `Self`, hiding repeated boilerplate caused by lack of nested impls.
```ignore
impl_for! { SomeTrait }
impl_for! { SomeTrait: impl_body }
impl_for! { <'a, 2> SomeTrait<self2!()>: impl_body }
```
where `'a`, `2` and impl_body are all optional, `2` meaning 2<sup>nd</sup> generic Stringlet.
*/
macro_rules! impl_for {
    // Split this rule, otherwise compiler says optional <…> is ambiguous
    // $two should match nothing, but marks that 2 was matched.
    (<$($lt:lifetime)? $(,)? $(2 $($two:literal)?)?> $trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ $(2 $($two)?)? $($lt)?; $trait: $($($rest)+)?);
    };
    ($trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ ; $trait: $($($rest)+)?);
    };

    (@ $(2 $($two:literal)?)? $($lt:lifetime)?; $trait:ty: $($rest:tt)*) => {
        impl_for!(@@
            $(2 $($two)?)?
            $($lt)?
            [Kind: StringletKind, const SIZE: usize, const LEN: usize, const ALIGN: u8,]
            [StringletBase<Kind, SIZE, LEN, ALIGN>: ConfigBase<Kind, SIZE, LEN, ALIGN>,]
            $trait:
            $($rest)*
        );
    };
    (@@ 2 $($lt:lifetime)? [$($gen:tt)+] [$($where:tt)+] $trait:ty: $($rest:tt)*) => {
        impl_for!(@@
            $($lt)?
            [$($gen)+ Kind2: StringletKind, const SIZE2: usize, const LEN2: usize, const ALIGN2: u8]
            [$($where)+ self2!(): ConfigBase<Kind2, SIZE2, LEN2, ALIGN2>,]
            $trait:
            $($rest)*
        );
    };
    (@@ $($lt:lifetime)? [$($gen:tt)+] [$($where:tt)+] $trait:ty: $($rest:tt)*) => {
        impl<$($lt,)? $($gen)+> $trait
        for StringletBase<Kind, SIZE, LEN, ALIGN>
        where $($where)+
        {
            $($rest)*
        }
    };
}

pub(crate) use impl_for;
pub(crate) use self2;
