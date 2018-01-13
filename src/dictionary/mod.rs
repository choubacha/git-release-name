pub mod adverbs;
pub mod adjectives;
pub mod nouns;

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
}
