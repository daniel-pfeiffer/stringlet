//! `Display` and `Debug` for Stringlet

use super::Stringlet;
use crate::repr::*;

use core::fmt::{Debug, Display, Error, Formatter};

impl<const CAPACITY: usize, const FIXED: bool> Display for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.as_str())
    }
}

impl<const CAPACITY: usize, const FIXED: bool> Debug for Stringlet<CAPACITY, FIXED>
where
    Size<CAPACITY>: Config<CAPACITY>,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{} {{ ", std::any::type_name::<Self>(),)?;
        if fmt.alternate() {
            // SAFETY: str is guaranteed to be initialized and valid UTF-8 up to len()
            unsafe {
                write!(
                    fmt,
                    "SIZE: {}, repr: {:?}, raw: {:?}, ",
                    Self::SIZE,
                    self.repr,
                    self.raw
                )?;
                if option_env!("STRINGLET_RAW_DEBUG").is_none() {
                    let len = self.len();
                    write!(fmt, "len(): {len}, ")?;
                    if len < CAPACITY {
                        write!(fmt, "str: [{:?}", self.as_str())?;
                        for i in len..CAPACITY {
                            write!(fmt, ", 0b11_{:06b}", self.str[i] ^ super::TAIL_TAG)?;
                        }
                        write!(fmt, "]")?;
                    } else {
                        write!(fmt, "str: {:?}", self.as_str())?;
                    }
                } else if CAPACITY > 0 {
                    let last = self.last();
                    write!(
                        fmt,
                        "is_last: {}, last_payload: {}({1:08b}:{:08b})",
                        last >= super::TAIL_TAG,
                        last,
                        last ^ super::TAIL_TAG
                    )?;
                }
            }
        } else {
            write!(fmt, "str: {:?}", self.as_str())?;
        }
        write!(fmt, " }}")
    }
}
