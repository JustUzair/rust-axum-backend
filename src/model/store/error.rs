use core::fmt::{Display, Formatter};

#[derive(Debug, serde::Serialize)]
pub enum Error {
    FailToCreatePool(String),
}
pub type Result<T> = core::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
