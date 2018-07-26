extern crate inflector;

mod adjectives;
mod adverbs;
mod case;
mod nouns;
mod phrase;
mod sha;

pub use self::case::Case;
pub use self::phrase::{ParsePhraseError, Phrase};

/// Looks up a phrase from a given str slice. It should be able to look up
/// any sized string but only if it's a valid hexadecimal.
pub fn lookup(sha: &str) -> Result<Phrase, ParsePhraseError> {
    sha.parse()
}

/// The kind of word.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// Noun
    Noun,
    /// Adjective
    Adj,
    /// Adverb
    Adv,
}

/// A word entry in the dictionary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: Kind,
    pub word: String,
    pub index: usize,
}

/// Lists out the word for a particular kind of word.
pub fn list(kind: Kind) -> Vec<Entry> {
    let list = match kind {
        Kind::Noun => &nouns::WORDS[..],
        Kind::Adv => &adverbs::WORDS[..],
        Kind::Adj => &adjectives::WORDS[..],
    };

    list.iter()
        .map(|s| String::from(*s))
        .enumerate()
        .map(|(index, word)| Entry { kind, index, word })
        .collect()
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
    fn listing() {
        assert_eq!(list(Kind::Noun).len(), 4096);
        assert_eq!(
            list(Kind::Noun)[0],
            Entry {
                word: String::from("kisses"),
                index: 0,
                kind: Kind::Noun
            }
        )
    }
}
