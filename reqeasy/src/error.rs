#[derive(Debug)]
pub struct Error {}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<::url::ParseError> for Error {
    fn from(_: ::url::ParseError) -> Error {
        //FIXME
        Error {}
    }
}

impl From<::std::io::Error> for Error {
    fn from(_: ::std::io::Error) -> Error {
        //FIXME
        Error {}
    }
}
