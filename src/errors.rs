#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    StringConvError(String),
    NullPtr,
}

pub type Result<T> = ::std::result::Result<T, Error>;
