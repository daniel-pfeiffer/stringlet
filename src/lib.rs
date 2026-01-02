#![doc = include_str!("../README.md")]
#![deny(clippy::alloc_instead_of_core, clippy::std_instead_of_core)]

/* #![no_std]
extern crate alloc; */

use core::marker::PhantomData;

mod cmp;
mod fmt;
mod macros;
mod methods;
mod new;
mod refs;
mod traits;

/**
Magic sauce for a UTF-8 hack: a byte containing two high bits is not a valid last byte.
Use this a a marker to distinguish whether we use the full length. Otherwise the lower bits contain
the length of the unused tail. At full length there is no tagged last byte, so we only need to encode
64 lengths. Which is where this crate’s length limit comes from.

To enable simple eq-test, always put the same value on all unused bytes! Counting from the end, i.e.
the length of the unused tail, makes the branchless implementation of `len()` more efficient.

If you change the semantics, `option_env!("STRINGLET_RAW_DEBUG")` is your friend.
*/
pub(crate) const TAG: u8 = 0b11_000000;

pub trait StringletKind {
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
pub trait ConfigBase<Kind, const SIZE: usize = 16, const LEN: usize = 0> {}

impl<const SIZE: usize> ConfigBase<Fixed, SIZE> for Stringlet<SIZE> {}

#[diagnostic::on_unimplemented(
    message = "`VarStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=255`",
    note = "`VarStringlet` cannot be longer than 255 bytes. Consider using `String`!"
)]
pub trait VarConfig<const SIZE: usize> {}
// VarConfig implemented by macro below
impl<const SIZE: usize> ConfigBase<Var, SIZE, 1> for VarStringlet<SIZE> where Self: VarConfig<SIZE> {}

impl<const SIZE: usize> ConfigBase<Trim, SIZE> for TrimStringlet<SIZE> {}

#[diagnostic::on_unimplemented(
    message = "`SlimStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=64`",
    note = "`SlimStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!"
)]
pub trait SlimConfig<const SIZE: usize> {}
// SlimConfig implemented by macro below
impl<const SIZE: usize> ConfigBase<Slim, SIZE> for SlimStringlet<SIZE> where Self: SlimConfig<SIZE> {}

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
    ($msg1:literal $msg2:literal $stringlet:ident $kind:ident $len:literal $($kind_config:ident $size:tt)?) => {
        #[derive(Copy, Clone)]
        pub enum $kind {}

        impl StringletKind for $kind {
            const ABBR: u8 = stringify!($kind).as_bytes()[0];
        }

        #[doc = concat!($msg1, " length kind of stringlet", $msg2)]
        $(#[doc = concat!("\n\nWhen instatiating this with a generic `SIZE`, you need to add this bound:\n```ignore\nwhere ",
            stringify!($stringlet), "<SIZE>: ", stringify!($kind_config), "<SIZE>\n```")])?
        pub type $stringlet<const SIZE: usize = 16> = StringletBase<$kind, SIZE, $len>;
        $(config![@ $stringlet $kind_config $size];)?
    };
}

config!("Fixed" ", i.e. bounds for array access are compiled in, hence it is fast.

This is also produced by [`stringlet!(…)`](stringlet!()) without a kind specifier."
    Stringlet Fixed 0);
config!("Variable" ", with one extra byte for the length.
    Speed differs for some content processing, where SIMD gives an advantage for multiples of some power of 2, e.g.
    `VarStringlet<32>`. While for copying the advantage can be at one less, e.g. `VarStringlet<31>`. Size must be `0..=255`.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `var` or `v`."
    VarStringlet Var 1 VarConfig 255);
config!("Trimmed" ", which optionally trims one last byte, useful for codes
    with minimal length variation like [ISO 639](https://www.iso.org/iso-639-language-code). This is achieved by tagging
    an unused last byte with a UTF-8 niche. The length gets calculated branchlessly with very few ops.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `trim` or `t`."
    TrimStringlet Trim 0);
config!("Slim variable" ", uses a UTF-8 niche: It projects the length into 6 bits of the last byte, when content is less
    than full size. Length must be `0..=64`. Though it is done branchlessly, there are a few more ops for length calculation.
    Hence this is the slowest, albeit by a small margin.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `slim` or `s`."
    SlimStringlet Slim 0 SlimConfig 64);

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
# use stringlet::{VarStringlet, VarConfig, SlimStringlet, SlimConfig};
fn create<const SIZE: usize>()
where
    VarStringlet<SIZE>: VarConfig<SIZE>,
    SlimStringlet<SIZE>: SlimConfig<SIZE>,
{
    let var = VarStringlet::<SIZE>::from("var");
    let slim: SlimStringlet<SIZE> = "slim".into();
}
```
*/
#[derive(Copy, Clone)]
pub struct StringletBase<
    Kind: StringletKind,
    const SIZE: usize,
    // Have to make LEN explicit, as we can’t pick up a const from StringletKind.
    // We could put the whole [u8; LEN] into an associated type, but then it would be opaque to us.
    const LEN: usize = 0,
> {
    /// The actual payload – if it is shorter than SIZE, its last bytes will be tagged.
    pub(crate) str: [u8; SIZE],
    /// Limited by available constructors to either 0 or 1 byte for an explicit length.
    // str can’t be SIZE + LEN, as “generic parameters may not be used in const operations”
    pub(crate) len: [u8; LEN],
    pub(crate) _kind: PhantomData<Kind>,
}

/// A 2<sup>nd</sup> generic StringletBase
macro_rules! self2 {
    () => {
        StringletBase<Kind2, SIZE2, LEN2>
    };
}

/** Impl `SomeTrait` for `Self`, hiding repeated boilerplate caused by lack of nested impls.
```ignore
impl_for! { SomeTrait }
impl_for! { SomeTrait: impl_body }
impl_for! { <'a, 2> SomeTrait<self2!()>: impl_body }
```
where `'a`, `2` and impl_body are all optional, `2` meaning 2<sup>nd</sup> generic stringlet.
*/
macro_rules! impl_for {
    // Split this rule, otherwise compiler says optional <…> is ambiguous
    // $two should match nothing, but marks that 2 was matched.
    (<$($lt:lifetime)? $(,)? $(2 $($two:literal)?)?> $trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ $(2 $($two)?)? $($lt)?; $trait: $($($rest)+)?);
    };
    (bound $trait:ty: $($rest:tt)+) => {
        impl<Kind: StringletKind, const SIZE: usize, const LEN: usize> $trait
        for StringletBase<Kind, SIZE, LEN>
        where Self: ConfigBase<Kind, SIZE, LEN>
        {
            $($rest)*
        }
    };
    ($trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ ; $trait: $($($rest)+)?);
    };

    (@ $(2 $($two:literal)?)? $($lt:lifetime)?; $trait:ty: $($rest:tt)*) => {
        impl_for!(@@
            $(2 $($two)?)?
            $($lt)?
            [Kind: StringletKind, const SIZE: usize, const LEN: usize,]
            $trait:
            $($rest)*
        );
    };
    (@@ 2 $($lt:lifetime)? [$($gen:tt)+] $trait:ty: $($rest:tt)*) => {
        impl_for!(@@
            $($lt)?
            [$($gen)+ Kind2: StringletKind, const SIZE2: usize, const LEN2: usize]
            $trait:
            $($rest)*
        );
    };
    (@@ $($lt:lifetime)? [$($gen:tt)+] $trait:ty: $($rest:tt)*) => {
        impl<$($lt,)? $($gen)+> $trait
        for StringletBase<Kind, SIZE, LEN>
        {
            $($rest)*
        }
    };
}

pub(crate) use impl_for;
pub(crate) use self2;
