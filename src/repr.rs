//! Cheap Copy & Eq representation to overlay on Stringlet.

use core::ops::Index;
use core::{fmt::Debug, hash::Hash};

// This is generic only for the benefit of the following message. Alas that pollutes into all impls.
// Repeated twice, as message alone can’t be cfg-ed
#[cfg(not(feature = "len64"))]
#[diagnostic::on_unimplemented(
    message = "`Stringlet<{CAPACITY}>` has excessive CAPACITY",
    label = "CAPACITY must be `0..=16`",
    note = "`Stringlet` cannot be longer than 16 bytes.",
    note = "Consider activating crate feature `len64` or using `String`!"
)]
pub trait Config<const CAPACITY: usize> {
    type Repr: Copy + Eq + Hash + Debug;
    type Raw: Copy + Eq + Index<usize, Output = u8> + Debug;
}

#[cfg(feature = "len64")]
#[diagnostic::on_unimplemented(
    message = "`Stringlet<{CAPACITY}>` has excessive CAPACITY",
    label = "CAPACITY must be `0..=64`",
    note = "`Stringlet` cannot be longer than 64 bytes. Consider using `String`!"
)]
pub trait Config<const CAPACITY: usize> {
    type Repr: Copy + Eq + Hash + Debug;
    type Raw: Copy + Eq + Index<usize, Output = u8> + Debug;
}

pub struct Size<const CAPACITY: usize>;

/// Implement our 2 Config GATs only for our desired sizes.
/// Can’t be more than 64 bytes, as we only have a 6 bit niche for length.
macro_rules! config {
    ($($capacity:literal),+ => $ty:ty) => {
        $(
            impl Config<$capacity> for Size<$capacity> {
                type Repr = $ty;
                type Raw = [u8; size_of::<$ty>()];
            }
        )+
    };
}

config!(0 => ());
config!(1 => u8);
config!(2 => u16);

#[cfg(not(feature = "len64"))]
mod len16 {
    use super::*;
    config!(3, 4 => u32);
    config!(5, 6, 7, 8 => u64);
    config!(9, 10, 11, 12, 13, 14, 15, 16 => u128);
}

// todo make this the standard if it is safe
// - It is not clear whether usize always has the native pointer or data size. On small CPUs this is not the same thing!
//   Since usize holds data, it should have the data bus width. But there is no cfg(target_data_width) to check!
// - On my PC (u64, u8) is not 9 but 16 bytes long. To avoid risking gaps or UB, I use only multiples of target_pointer_width.
// - On my PC tuples seem to be packed, and despite the doc #[repr(packed)] seems to break alignment…
#[cfg(feature = "len64")]
mod len64 {
    use super::*;

    #[cfg(target_pointer_width = "16")]
    mod u16 {
        use super::*;
        // need to do sub-grouping, as Debug is only implemented up to 12-tuples
        type U16_4 = (u16, u16, u16, u16);
        config!(3, 4 => (u16, u16)); // todo is this better than u32 on such an architecture?
        config!(5, 6 => (u16, u16, u16));
        config!(7, 8 => U16_4);
        config!(9, 10 => (U16_4, u16));
        config!(11, 12 => (U16_4, u16, u16));
        config!(13, 14 => (U16_4, u16, u16, u16));
        config!(15, 16 => (U16_4, U16_4));
        config!(17, 18 => (U16_4, U16_4, u16));
        config!(19, 20 => (U16_4, U16_4, u16, u16));
        config!(21, 22 => (U16_4, U16_4, u16, u16, u16));
        config!(23, 24 => (U16_4, U16_4, U16_4));
        config!(25, 26 => (U16_4, U16_4, U16_4, u16));
        config!(27, 28 => (U16_4, U16_4, U16_4, u16, u16));
        config!(29, 30 => (U16_4, U16_4, U16_4, u16, u16, u16));
        config!(31, 32 => (U16_4, U16_4, U16_4, U16_4));
        config!(33, 34 => (U16_4, U16_4, U16_4, U16_4, u16));
        config!(35, 36 => (U16_4, U16_4, U16_4, U16_4, u16, u16));
        config!(37, 38 => (U16_4, U16_4, U16_4, U16_4, u16, u16, u16));
        config!(39, 40 => (U16_4, U16_4, U16_4, U16_4, U16_4));
        config!(41, 42 => (U16_4, U16_4, U16_4, U16_4, U16_4, u16));
        config!(43, 44 => (U16_4, U16_4, U16_4, U16_4, U16_4, u16, u16));
        config!(45, 46 => (U16_4, U16_4, U16_4, U16_4, U16_4, u16, u16, u16));
        config!(47, 48 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4));
        config!(49, 50 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, u16));
        config!(51, 52 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, u16, u16));
        config!(53, 54 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, u16, u16, u16));
        config!(55, 56 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, U16_4));
        config!(57, 58 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, u16));
        config!(59, 60 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, u16, u16));
        config!(61, 62 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, u16, u16, u16));
        config!(63, 64 => (U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, U16_4, U16_4));
    }

    #[cfg(target_pointer_width = "32")]
    mod u32 {
        use super::*;
        type U32_4 = (u32, u32, u32, u32);
        config!(3, 4 => u32);
        config!(5, 6, 7, 8 => (u32, u32)); // todo is this better than u64 on such an architecture?
        config!(9, 10, 11, 12 => (u32, u32, u32));
        config!(13, 14, 15, 16 => U32_4);
        config!(17, 18, 19, 20 => (U32_4, u32));
        config!(21, 22, 23, 24 => (U32_4, u32, u32));
        config!(25, 26, 27, 28 => (U32_4, u32, u32, u32));
        config!(29, 30, 31, 32 => (U32_4, U32_4));
        config!(33, 34, 35, 36 => (U32_4, U32_4, u32));
        config!(37, 38, 39, 40 => (U32_4, U32_4, u32, u32));
        config!(41, 42, 43, 44 => (U32_4, U32_4, u32, u32, u32));
        config!(45, 46, 47, 48 => (U32_4, U32_4, U32_4));
        config!(49, 50, 51, 52 => (U32_4, U32_4, U32_4, u32));
        config!(53, 54, 55, 56 => (U32_4, U32_4, U32_4, u32, u32));
        config!(57, 58, 59, 60 => (U32_4, U32_4, U32_4, u32, u32, u32));
        config!(61, 62, 63, 64 => (U32_4, U32_4, U32_4, U32_4));
    }

    #[cfg(target_pointer_width = "64")]
    mod u64 {
        use super::*;
        config!(3, 4 => u32);
        config!(5, 6, 7, 8 => u64);
        config!(9, 10, 11, 12, 13, 14, 15, 16 => (u64, u64));
        config!(17, 18, 19, 20, 21, 22, 23, 24 => (u64, u64, u64));
        config!(25, 26, 27, 28, 29, 30, 31, 32 => (u64, u64, u64, u64));
        config!(33, 34, 35, 36, 37, 38, 39, 40 => (u64, u64, u64, u64, u64));
        config!(41, 42, 43, 44, 45, 46, 47, 48 => (u64, u64, u64, u64, u64, u64));
        config!(49, 50, 51, 52, 53, 54, 55, 56 => (u64, u64, u64, u64, u64, u64, u64));
        config!(57, 58, 59, 60, 61, 62, 63, 64 => (u64, u64, u64, u64, u64, u64, u64, u64));
    }
}

pub(crate) type Repr<const CAPACITY: usize> = <Size<CAPACITY> as Config<CAPACITY>>::Repr;
pub(crate) type Raw<const CAPACITY: usize> = <Size<CAPACITY> as Config<CAPACITY>>::Raw;
