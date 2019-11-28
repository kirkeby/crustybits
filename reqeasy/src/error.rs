#[derive(Debug)]
pub struct Error {}
// Box<Err>?!

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        //FIXME
        Error {}
    }
}
