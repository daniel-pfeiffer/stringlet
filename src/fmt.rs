//! `Display` and `Debug` for Stringlet

use crate::*;

use core::fmt::{Debug, Display, Error, Formatter};

impl_for! {
    Display:

    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.as_str())
    }
}

impl_for! {
    Debug:

    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        if fmt.alternate() {
            write!(
                fmt,
                "{} {{ /* TODO adapt {{:#?}} to 4 kinds */ ",
                core::any::type_name::<Self>(),
            )?;
            let len = self.len();
            write!(fmt, "SIZE: {}, len(): {len}, [u8]: {:?}, ", SIZE, self.str)?;
            if option_env!("STRINGLET_RAW_DEBUG").is_none() {
                if len < SIZE {
                    write!(fmt, "str: [{:?}", self.as_str())?;
                    for i in len..SIZE {
                        write!(fmt, ", 0b11_{:06b}", self.str[i] ^ TAG)?;
                    }
                    write!(fmt, "]")?;
                } else {
                    write!(fmt, "str: {:?}", self.as_str())?;
                }
            } else if SIZE > 0 {
                let last = self.last();
                if last >= TAG {
                    write!(
                        fmt,
                        "last_tagged: ({}, {0:08b}; {}, {1:06b})",
                        last,
                        last ^ TAG
                    )?;
                }
            }
        } else {
            write!(
                fmt,
                "{} {{ str: {:?}",
                Self::type_name(),
                self.as_str()
            )?;
        }
        write!(fmt, " }}")
    }
}
