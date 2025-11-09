//! `Display` and `Debug` for Stringlet

use crate::{methods::TAIL_TAG, *};

use core::fmt::{Debug, Display, Error, Formatter};

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> Display
    for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.as_str())
    }
}

impl<const SIZE: usize, const FIXED: bool, const ALIGN: u8> Debug for Stringlet<SIZE, FIXED, ALIGN>
where
    Self: Config<SIZE, ALIGN>,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        if fmt.alternate() {
            write!(fmt, "{} {{ ", std::any::type_name::<Self>(),)?;
            let len = self.len();
            write!(fmt, "SIZE: {}, len(): {len}, [u8]: {:?}, ", SIZE, o!(self))?;
            if option_env!("STRINGLET_RAW_DEBUG").is_none() {
                if len < SIZE {
                    write!(fmt, "str: [{:?}", self.as_str())?;
                    for i in len..SIZE {
                        write!(fmt, ", 0b11_{:06b}", o!(self)[i] ^ TAIL_TAG)?;
                    }
                    write!(fmt, "]")?;
                } else {
                    write!(fmt, "str: {:?}", self.as_str())?;
                }
            } else if SIZE > 0 {
                let last = self.last();
                if last >= TAIL_TAG {
                    write!(
                        fmt,
                        "last_tagged: ({}, {0:08b}; {}, {1:06b})",
                        last,
                        last ^ TAIL_TAG
                    )?;
                }
            }
        } else {
            write!(fmt, "{} {{ str: {:?}", Self::type_name(), self.as_str())?;
        }
        write!(fmt, " }}")
    }
}
