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
                "{} {{ '{}' ",
                core::any::type_name::<Self>(),
                Kind::ABBR as char
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

impl Display for error::Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            TooLong => write!(fmt, "too long"),
            TooShort => write!(fmt, "too short"),
            Utf8Error(e) => write!(fmt, "{e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn error_display() {
        assert_eq!(
            Stringlet::<1>::from_str("").unwrap_err().to_string(),
            "too short"
        );
        assert_eq!(
            Stringlet::<0>::from_str("a").unwrap_err().to_string(),
            "too long"
        );
    }
}
