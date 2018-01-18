mod adverbs;
mod adjectives;
mod nouns;
mod case;
mod sha_result;
mod sha_part;
mod phrase;

pub use self::case::Case;

use self::phrase::Phrase;
use self::sha_result::{ShaResult, ParseShaError};
use self::sha_part::ShaPart;

/// Looks up a phrase from a given str slice. It should be able to look up
/// any sized string but only if it's a valid hexadecimal.
pub fn lookup(sha: &str) -> ShaResult<Phrase> {
    sha.parse()
}

fn lookup_adverb(part: ShaPart) -> ShaResult<String> {
    adverbs::WORDS
        .get(part.hash() % adverbs::WORDS.len())
        .map(|s| s.to_string())
        .ok_or(ParseShaError::WordNotFound)
}

fn lookup_adjective(part: ShaPart) -> ShaResult<String> {
    adjectives::WORDS
        .get(part.hash() % adjectives::WORDS.len())
        .map(|s| s.to_string())
        .ok_or(ParseShaError::WordNotFound)
}

fn lookup_noun(part: ShaPart) -> ShaResult<String> {
    nouns::WORDS
        .get(part.hash() % nouns::WORDS.len())
        .map(|s| s.to_string())
        .ok_or(ParseShaError::WordNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;

    fn has_unique_elements<T>(iter: T) -> bool
        where
            T: IntoIterator,
            T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    #[test]
    fn unique_function_detects_non_unique() {
        assert!(!has_unique_elements(vec![1, 1].iter()));
    }

    #[test]
    fn adverbs_are_unique() {
        assert!(has_unique_elements(adverbs::WORDS.iter()));
    }

    #[test]
    fn adjectives_are_unique() {
        assert!(has_unique_elements(adjectives::WORDS.iter()));
    }

    #[test]
    fn nouns_are_unique() {
        assert!(has_unique_elements(nouns::WORDS.iter()));
    }

    #[test]
    fn it_can_look_up_from_an_adverb() {
        let word = lookup_adverb("1".parse().unwrap());
        assert_eq!(word, Ok("exaggeratedly".to_string()));
        let word = lookup_adverb("ffff".parse().unwrap());
        assert_eq!(word, Ok("disconcertingly".to_string()));
    }

    #[test]
    fn it_can_look_up_an_adjective() {
        let word = lookup_adjective("1".parse().unwrap());
        assert_eq!(word, Ok("courant".to_string()));
        let word = lookup_adjective("ffff".parse().unwrap());
        assert_eq!(word, Ok("gleeful".to_string()));
    }

    #[test]
    fn it_can_look_up_a_noun() {
        let word = lookup_noun("1".parse().unwrap());
        assert_eq!(word, Ok("ombre".to_string()));
        let word = lookup_noun("ffff".parse().unwrap());
        assert_eq!(word, Ok("tipsters".to_string()));
    }
}
