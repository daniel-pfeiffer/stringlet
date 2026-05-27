//! All implementations dealing with refs.

use crate::*;

use core::ops::Deref;

impl_for! {
    Deref:

    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl_for! {
    AsRef<str>:

    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

macro_rules! impl_ref {
    ($type:ty => $stringlet:ident $(: $conf:tt)?) => {
        impl<const SIZE: usize> AsRef<$stringlet<SIZE>> for $type
        $(where
            $stringlet<SIZE>: $conf<SIZE>)?
        {
            #[doc = concat!("Cast a `", stringify!($type), "` as a shared reference to a [`", stringify!($stringlet), "`].")]
            /// The size may be inferred, but it must match the input length!
            #[inline]
            fn as_ref(&self) -> &$stringlet<SIZE> {
                assert_eq!(
                    self.len(),
                    SIZE,
                    concat!("Cannot cast a len {} ", stringify!($type), " as &", stringify!($stringlet), "<SIZE>"),
                    self.len()
                );
                // SAFETY I’m not sure. It seems to cast the input bytes to stringlet just fine.
                unsafe { core::mem::transmute(&*(self.as_ptr() as *const $stringlet<SIZE>)) }
            }
        }
    };
}

impl_ref!(str => Stringlet);
impl_ref!(str => VarStringlet: VarConfig);
impl_ref!(str => TrimStringlet);
impl_ref!(str => SlimStringlet: SlimConfig);

impl_ref!(String => Stringlet);
impl_ref!(String => VarStringlet: VarConfig);
impl_ref!(String => TrimStringlet);
impl_ref!(String => SlimStringlet: SlimConfig);

impl_ref!(Box<str> => Stringlet);
impl_ref!(Box<str> => VarStringlet: VarConfig);
impl_ref!(Box<str> => TrimStringlet);
impl_ref!(Box<str> => SlimStringlet: SlimConfig);
