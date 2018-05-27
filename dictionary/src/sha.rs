use std::str::FromStr;

/// Represents a sha. Provides convenience functions for library
/// indexes.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Sha(u32);

/// Error types for parsing a sha into a phrase/word.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ParseShaError {
    /// The sha had non-hex characters in it
    NonHexadecimalCharacters,
    #[doc(hidden)]
    __NonExhaustive,
}

impl FromStr for Sha {
    type Err = ParseShaError;

    fn from_str(sha: &str) -> Result<Sha, Self::Err> {
        if let Ok(hash) = u32::from_str_radix(&sha, 16) {
            Ok(Sha(hash))
        } else {
            Err(ParseShaError::NonHexadecimalCharacters)
        }
    }
}

const NIBBLES: u32 = 4;

impl Sha {
    /// Returns the adverb index for this sha
    pub fn adverb(&self) -> usize {
        const ADV_MASK: u32 = 0xfff00000;
        ((self.0 & ADV_MASK) >> (5 * NIBBLES)) as usize
    }

    /// Returns the adjective index for this sha
    pub fn adjective(&self) -> usize {
        const ADJ_MASK: u32 = 0x000ff000;
        ((self.0 & ADJ_MASK) >> (3 * NIBBLES)) as usize
    }

    /// Returns the noun index for this sha
    pub fn noun(&self) -> usize {
        const NOUN_MASK: u32 = 0x00000fff;
        (self.0 & NOUN_MASK) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_into_a_sha() {
        assert_eq!("a".parse::<Sha>().unwrap().0, 10);
    }

    #[test]
    fn it_can_detect_non_hex_chars_when_parsing() {
        assert!("z".parse::<Sha>().is_err());
    }

    #[test]
    fn it_can_identify_indexes_for_each_type() {
        let sha = Sha(0xffffffff);
        assert_eq!(sha.adverb(), 4095);
        assert_eq!(sha.adjective(), 255);
        assert_eq!(sha.noun(), 4095);

        let sha = Sha(0xfff00000);
        assert_eq!(sha.adverb(), 4095);
        assert_eq!(sha.adjective(), 0);
        assert_eq!(sha.noun(), 0);

        let sha = Sha(0x00ff000);
        assert_eq!(sha.adverb(), 0);
        assert_eq!(sha.adjective(), 255);
        assert_eq!(sha.noun(), 0);

        let sha = Sha(0x00000fff);
        assert_eq!(sha.adverb(), 0);
        assert_eq!(sha.adjective(), 0);
        assert_eq!(sha.noun(), 4095);
    }
}
