#[derive(Debug, Eq, PartialEq)]
pub enum ParseShaError {
    NonHexadecimalCharacters,
    WordNotFound,
}

pub type ShaResult<T> = Result<T, ParseShaError>;
