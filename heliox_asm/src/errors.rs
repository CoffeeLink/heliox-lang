
pub enum ParseError {
    UnexpectedTokenError
}

pub enum Error {
    ParseError(ParseError)
}