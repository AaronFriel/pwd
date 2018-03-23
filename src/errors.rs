#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    StringConvError(String),
}

pub type Result<T> = ::std::result::Result<T, Error>;
