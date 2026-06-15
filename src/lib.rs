#![doc = include_str!("../README.md")]
#![deny(clippy::alloc_instead_of_core, clippy::std_instead_of_core)]
#![allow(clippy::wildcard_imports)] // only our own

/* #![no_std]
extern crate alloc; */

use core::marker::PhantomData;

mod cmp;
pub mod error;
mod fmt;
mod macros;
mod methods;
mod new;
pub mod prelude;
mod refs;
mod traits;
mod workaround;

pub(crate) use error::Error::*;
pub type Result<T> = core::result::Result<T, error::Error>;

/**
Magic sauce for a UTF-8 hack: a byte containing two high bits is not a valid last byte.
Use this a a marker to distinguish whether we use the full length. Otherwise the lower bits contain
the length of the unused tail. At full length there is no tagged last byte, so we only need to encode
64 lengths. Which is where this crate’s length limit comes from.

To enable simple eq-test, always put the same value on all unused bytes! Counting from the end, i.e.
the length of the unused tail, makes the branchless implementation of `len()` more efficient.
*/
pub(crate) const TAG: u8 = 0b11_000000;

pub trait Kind {
    type ExtraLen: Copy + Clone;
    const FIXED: bool = false;
    const VAR: bool = false;
    const TRIM: bool = false;
    const SLIM: bool = false;
    const NAME: &str;
    const ABBR: u8;
}

/// Configure constructors of `StringletBase` to have only valid generic parameters.
#[diagnostic::on_unimplemented(
    message = "`VarStringlet<{SIZE}>` or `SlimStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=255` or `0..=64`",
    note = "`VarStringlet` cannot be longer than 255 bytes. Consider using `String`!",
    note = "`SlimStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!"
)]
#[doc(hidden)]
pub trait Config<Kind, const SIZE: usize = 16> {}

impl<const SIZE: usize> Config<Fixed, SIZE> for Stringlet<SIZE> {}

#[diagnostic::on_unimplemented(
    message = "`VarStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=255`",
    note = "`VarStringlet` cannot be longer than 255 bytes. Consider using `String`!"
)]
pub trait VarConfig<const SIZE: usize> {}
// VarConfig implemented by macro below
impl<const SIZE: usize> Config<Var, SIZE> for VarStringlet<SIZE> where Self: VarConfig<SIZE> {}

impl<const SIZE: usize> Config<Trim, SIZE> for TrimStringlet<SIZE> {}

#[diagnostic::on_unimplemented(
    message = "`SlimStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=64`",
    note = "`SlimStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!"
)]
pub trait SlimConfig<const SIZE: usize> {}
// SlimConfig implemented by macro below
impl<const SIZE: usize> Config<Slim, SIZE> for SlimStringlet<SIZE> where Self: SlimConfig<SIZE> {}

macro_rules! config {
    (@@ $stringlet:ident $kind_config:ident $($size:tt)+) => {
        $(
            impl $kind_config<$size> for $stringlet<$size> {}
        )+
    };
    (@ $stringlet:ident) => {};
    (@ $stringlet:ident $kind_config:ident 64) => {
        config![@@ $stringlet $kind_config
            // for VarStringlet and SlimStringlet
            0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
            32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60
            61 62 63 64
        ];
    };
    (@ $stringlet:ident $kind_config:ident 255) => {
        config![@ $stringlet $kind_config 64];
        config![@@ $stringlet $kind_config
            // for VarStringlet
            65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93
            94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116
            117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138
            139 140 141 142 143 144 145 146 147 148 149 150 151 152 153 154 155 156 157 158 159 160
            161 162 163 164 165 166 167 168 169 170 171 172 173 174 175 176 177 178 179 180 181 182
            183 184 185 186 187 188 189 190 191 192 193 194 195 196 197 198 199 200 201 202 203 204
            205 206 207 208 209 210 211 212 213 214 215 216 217 218 219 220 221 222 223 224 225 226
            227 228 229 230 231 232 233 234 235 236 237 238 239 240 241 242 243 244 245 246 247 248
            249 250 251 252 253 254 255
        ];
    };
    ($msg1:literal $msg2:literal $stringlet:ident $kind:ident $const:ident $extra_len:tt $($kind_config:ident $size:tt)?) => {
        #[derive(Copy, Clone)]
        pub enum $kind {}

        impl Kind for $kind {
            const $const: bool = true;
            type ExtraLen = $extra_len;
            const NAME: &str = stringify!($stringlet);
            const ABBR: u8 = stringify!($kind).as_bytes()[0]; // todo NAME[0]
        }

        #[doc = concat!($msg1, " length kind of stringlet", $msg2)]
        $(#[doc = concat!("\n\nWhen instatiating this with a generic `SIZE`, you need to add this bound:\n```ignore\nwhere ",
            stringify!($stringlet), "<SIZE>: ", stringify!($kind_config), "<SIZE>\n```")])?
        pub type $stringlet<const SIZE: usize = 16> = StringletBase<$kind, SIZE>;
        $(config![@ $stringlet $kind_config $size];)?
    };
}

config!("Fixed" ", i.e. bounds for array access are compiled in, hence it is fast.

This is also produced by [`stringlet!(…)`](stringlet!()) without a kind specifier."
    Stringlet Fixed FIXED ());
config!("Variable" ", with one extra byte for the length.
    Speed differs for some content processing, where SIMD gives an advantage for multiples of some power of 2, e.g.
    `VarStringlet<32>`. While for copying the advantage can be at one less, e.g. `VarStringlet<31>`. Size must be `0..=255`.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `var` or `v`."
    VarStringlet Var VAR u8 VarConfig 255);
config!("Trimmed" ", which optionally trims one last byte, useful for codes
    with minimal length variation like [ISO 639](https://www.iso.org/iso-639-language-code). This is achieved by tagging
    an unused last byte with a UTF-8 niche. The length gets calculated branchlessly with very few ops.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `trim` or `t`."
    TrimStringlet Trim TRIM ());
config!("Slim variable" ", uses a UTF-8 niche: It projects the length into 6 bits of the last byte, when content is less
    than full size. Length must be `0..=64`. Though it is done branchlessly, there are a few more ops for length calculation.
    Hence this is the slowest, albeit by a small margin.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `slim` or `s`."
    SlimStringlet Slim SLIM () SlimConfig 64);

/** An inline String of varying size bounds, which can be handled like a primitive type.
This is the underlying type, which you would not use directly. Instead use one of:

- **[`Stringlet`], [`stringlet!(…)`](stringlet!())**: This is fixed size, i.e. bounds for array access are compiled in,
  hence fast.

- **[`VarStringlet`], `stringlet!(var …)`, `stringlet!(v …)`**: This adds one byte for the length – still pretty fast.
  Speed differs for some content processing, where SIMD gives an advantage for multiples of some power of 2, e.g.
  `VarStringlet<32>`. While for copying the advantage can be at one less, e.g. `VarStringlet<31>`. Size must be `0..=255`.

- **[`TrimStringlet`], `stringlet!(trim …)`, `stringlet!(t …)`**: This can optionally trim one last byte, useful for codes
  with minimal length variation like [ISO 639](https://www.iso.org/iso-639-language-code). This is achieved by tagging
  an unused last byte with a UTF-8 niche. The length gets calculated branchlessly with very few ops.

- **[`SlimStringlet`], `stringlet!(slim …)`, `stringlet!(s …)`**: This uses the same UTF-8 niche, but fully: It projects
  the length into 6 bits of the last byte, when content is less than full size. Length must be `0..=64`. Though it is
  done branchlessly, there are a few more ops for length calculation. Hence this is the slowest, albeit by a small
  margin.

If you want to create either of `VarStringlet` or `SlimStringlet` generically, you must specify their bounds:
```
use stringlet::{VarStringlet, VarConfig, SlimStringlet, SlimConfig, Result};
fn create<const SIZE: usize>() -> Result<()>
where
    VarStringlet<SIZE>: VarConfig<SIZE>,
    SlimStringlet<SIZE>: SlimConfig<SIZE>,
{
    let _var = VarStringlet::<SIZE>::try_from("var")?;
    let _slim: SlimStringlet<SIZE> = "slim".try_into()?;
    Ok(())
}
_ = create::<5>();
```
*/
// Workaround for [u8; SIZE + Kind::EXTRA_LEN], as long as “generic parameters may not be used in const operations”
// Adapted from CAD97 https://internals.rust-lang.org/t/what-s-where-size-kind-extra/23987/9
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StringletBase<Kind: crate::Kind, const SIZE: usize> {
    /// The actual payload – if it is shorter than SIZE, its last bytes will be tagged.
    pub(crate) str: [u8; SIZE],
    pub(crate) extra_len: Kind::ExtraLen,
    pub(crate) _kind: PhantomData<Kind>,
}

/** Impl `SomeTrait` for `Self`, hiding repeated boilerplate caused by lack of nested impls.
```ignore
impl_for! { SomeTrait }
impl_for! { SomeTrait: impl_body }
impl_for! { <'a, Config, 2> SomeTrait<self2!()>: impl_body }
```
where `'a`, `Config`, `2` and impl_body are all optional, `2` meaning 2<sup>nd</sup> generic stringlet.
*/
macro_rules! impl_for {
    // Split this rule, otherwise compiler says optional <…> is ambiguous
    // $two should match nothing, but marks that 2 was matched.
    (<$($lt:lifetime $(,)?)? $($config:ident $(,)?)? $(2 $($two:literal)?)?> $trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ $(2 $($two)?)? ($($lt)?) {$($config)?} [] $trait: $($($rest)+)?);
    };
    ($trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ () {} [] $trait: $($($rest)+)?);
    };

    (@ ($($lt:lifetime)?) {$($config:ident)?} [$($gen:tt)*] $trait:ty: $($rest:tt)*) => {
        impl<$($lt,)? Kind: crate::Kind, const SIZE: usize, $($gen)*> $trait
        for StringletBase<Kind, SIZE>
        $(where Self: $config<Kind, SIZE>)?
        //where Self: Config<Kind, SIZE>
        {
            $($rest)*
        }
    };
    (@ 2 $lt:tt $config:tt [] $trait:ty: $($rest:tt)*) => {
        impl_for!(@
            $lt
            $config
            [Kind2: crate::Kind, const SIZE2: usize]
            $trait:
            $($rest)*
        );
    };
}

/// A 2<sup>nd</sup> generic `StringletBase`.
macro_rules! self2 {
    () => {
        StringletBase<Kind2, SIZE2>
    };
}

pub(crate) use impl_for;
pub(crate) use self2;
