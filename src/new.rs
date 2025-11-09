//! Stringlet `new()` and `default()` only where they make sence.

use crate::*;

// These are twice twice identical but for different generic constellations.
// A FixedStringlet can only be empty for SIZE 0.
impl<const ALIGN: u8> Stringlet<0, true, ALIGN>
where
    Self: Config<0, ALIGN>,
{
    pub const fn new() -> Self {
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }
}

// A sized Stringlet can only be empty if not FIXED.
impl<const SIZE: usize, const ALIGN: u8> Stringlet<SIZE, false, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    pub const fn new() -> Self {
        // SAFETY always short enough and no bytes that can have a UTF-8 error
        unsafe { Self::from_utf8_unchecked(&[]) }
    }
}

impl<const ALIGN: u8> Default for Stringlet<0, true, ALIGN>
where
    Self: Config<0, ALIGN>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize, const ALIGN: u8> Default for Stringlet<SIZE, false, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn default() -> Self {
        Self::new()
    }
}
