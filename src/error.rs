use core::str::Utf8Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The stringlet is too long to fit in the given size.
    TooLong,
    /// The stringlet is too short to be valid.
    TooShort,
    Utf8Error(Utf8Error),
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}
