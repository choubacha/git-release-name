use std::str::FromStr;
use dictionary::sha_result::{ParseShaError, ShaResult};

#[derive(Debug)]
pub struct ShaPart {
    sha: String,
    hash: usize,
}

impl ShaPart {
    pub fn hash(&self) -> usize {
        self.hash
    }
}

impl FromStr for ShaPart {
    type Err = ParseShaError;

    fn from_str(sha: &str) -> ShaResult<ShaPart> {
        if let Ok(hash) = usize::from_str_radix(&sha, 16) {
            Ok(ShaPart { sha: sha.to_string(), hash })
        } else {
            Err(ParseShaError::NonHexadecimalCharacters)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_into_a_word() {
        assert_eq!("a".parse::<ShaPart>().unwrap().hash(), 10);
    }

    #[test]
    fn it_can_detect_non_hex_chars() {
        assert!("z".parse::<ShaPart>().is_err());
    }
}
