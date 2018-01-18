use std::fmt::{Display, Formatter, Error};
use std::str::FromStr;
use dictionary::sha_result::{ParseShaError, ShaResult};
use dictionary::case::{Case};
use dictionary;

/// A phrase that is made up of an adverb, adjective, noun.
///
/// When parsed from a slice it will lookup the sha parts in the dictionary.
/// It knows how to properly format itself if a different case is selected.
pub struct Phrase {
    adj: String,
    adv: String,
    noun: String,
    format: Case,
}

impl Phrase {
    pub fn with_case(mut self, f: Case) -> Self {
        self.format = f;
        self
    }
}

impl FromStr for Phrase {
    type Err = ParseShaError;

    fn from_str(sha: &str) -> ShaResult<Phrase> {
        // Ensure that the sha is at least 8 characters so that
        // when we extract the first 8 there is something there.
        let sha = format!("{:0>8}", sha);
        let adv = dictionary::lookup_adverb(sha[0..3].parse()?)?;
        let adj = dictionary::lookup_adjective(sha[3..5].parse()?)?;
        let noun = dictionary::lookup_noun(sha[5..8].parse()?)?;

        Ok(Phrase {
            adv,
            adj,
            noun,
            format: Case::Lower,
        })
    }
}

impl Display for Phrase {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use inflector::Inflector;

        let ret = format!("{} {} {}", self.adv, self.adj, self.noun);
        match self.format {
            Case::Snake => write!(f, "{}", ret.to_snake_case()),
            Case::Kebab => write!(f, "{}", ret.to_kebab_case()),
            Case::Pascal => write!(f, "{}", ret.to_pascal_case()),
            Case::Camel => write!(f, "{}", ret.to_camel_case()),
            Case::Title => write!(f, "{}", ret.to_title_case()),
            Case::Sentence => write!(f, "{}", ret.to_sentence_case()),
            Case::Lower => write!(f, "{}", ret),
            Case::Upper => write!(f, "{}", ret.to_uppercase()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple_phrase() -> Phrase {
        "0a00a00a".parse::<Phrase>().expect("Invalid phrase")
    }

    #[test]
    fn a_phrase_can_be_generated_from_a_str() {
        let phrase = make_simple_phrase();
        assert_eq!("immeasurably endways borings", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_snake_case() {
        let phrase = make_simple_phrase().with_case(Case::Snake);
        assert_eq!("immeasurably_endways_borings", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_kebab_case() {
        let phrase = make_simple_phrase().with_case(Case::Kebab);
        assert_eq!("immeasurably-endways-borings", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_camel_case() {
        let phrase = make_simple_phrase().with_case(Case::Camel);
        assert_eq!("immeasurablyEndwaysBorings", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_pascal_case() {
        let phrase = make_simple_phrase().with_case(Case::Pascal);
        assert_eq!("ImmeasurablyEndwaysBorings", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_title_case() {
        let phrase = make_simple_phrase().with_case(Case::Title);
        assert_eq!("Immeasurably Endways Borings", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_capital_case() {
        let phrase = make_simple_phrase().with_case(Case::Sentence);
        assert_eq!("Immeasurably endways borings", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_upper_case() {
        let phrase = make_simple_phrase().with_case(Case::Upper);
        assert_eq!("IMMEASURABLY ENDWAYS BORINGS", format!("{}", phrase));
    }

    #[test]
    fn a_phrase_can_be_formatted_as_lower_case() {
        let phrase = make_simple_phrase().with_case(Case::Lower);
        assert_eq!("immeasurably endways borings", format!("{}", phrase));
    }
}
