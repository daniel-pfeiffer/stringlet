//! `Display` and `Debug` for stringlet

use crate::*;

use core::fmt::{Debug, Display, Formatter, Result};

impl_for! {
    Display:

    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "{}", self.as_str())
    }
}

impl_for! {
    Debug:

    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        if fmt.alternate() {
            write!(
                fmt,
                "{} {{ ",
                core::any::type_name::<Self>(),
            )?;
            let len = self.len();
            if len < SIZE {
                write!(fmt, "len(): {len}, ")?;
            }
            write!(fmt, "[u8]: {:?}, ", self.str)?;
            if len < SIZE {
                write!(fmt, "str: [{:?}", self.as_str())?;
                for i in len..SIZE-1 {
                    write!(fmt, ", {}", self.str[i])?;
                }
                if Kind::SLIM || Kind::TRIM {
                    write!(fmt, ", 0b11_{:06b}]", self.last() ^ TAG)?;
                } else {
                    write!(fmt, ", {}]", self.last())?;
                }
            } else {
                write!(fmt, "str: {:?}", self.as_str())?;
            }
        } else {
            write!(fmt, "{}", Kind::NAME)?;
            if SIZE != 16 {
                write!(fmt, "<{SIZE}>")?;
            }
            write!(
                fmt,
                " {{ str: {:?}",
                self.as_str()
            )?;
        }
        write!(fmt, " }}")
    }
}
