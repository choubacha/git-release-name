/// Error types for parsing a sha into a phrase/word.
#[derive(Debug, Eq, PartialEq)]
pub enum ParseShaError {
    NonHexadecimalCharacters,
    WordNotFound,
}

pub type ShaResult<T> = Result<T, ParseShaError>;
