extern crate rand;
extern crate atty;
extern crate clap;
extern crate inflector;

use std::fmt::{Display, Formatter, Error};
use std::io::{self, BufRead};
use atty::Stream;
use clap::{Arg, App, ArgMatches};
use std::str::FromStr;

mod dictionary;

#[derive(Debug, Eq, PartialEq)]
enum ParsePhraseError {
    NonHexadecimalCharacters,
}

type ShaResult<T> = Result<T, ParsePhraseError>;

#[derive(Debug)]
struct Word {
    sha: String,
    hash: usize,
    word: Option<String>,
}

fn lookup(word: Word, dict: &[&str]) -> Word {
    Word {
        word: dict.get(word.hash % dict.len()).map(|s| s.to_string()),
        ..word
    }
}

impl FromStr for Word {
    type Err = ParsePhraseError;

    fn from_str(sha: &str) -> ShaResult<Word> {
        if let Ok(hash) = usize::from_str_radix(&sha, 16) {
            Ok(Word {
                sha: sha.to_string(),
                hash,
                word: None,
            })
        } else {
            Err(ParsePhraseError::NonHexadecimalCharacters)
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if let Some(ref word) = self.word {
            write!(f, "{}", word)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Case {
    Snake,
    Kebab,
    Pascal,
    Camel,
    Title,
    Sentence,
    Lower,
    Upper,
}

#[derive(Debug, Eq, PartialEq)]
enum ParseCaseError {
    InvalidFormat,
}

impl FromStr for Case {
    type Err = ParseCaseError;

    fn from_str(format: &str) -> Result<Case, ParseCaseError> {
        let case = match format {
            "snake" => Case::Snake,
            "kebab" => Case::Kebab,
            "pascal" => Case::Pascal,
            "camel" => Case::Camel,
            "title" => Case::Title,
            "sentence" => Case::Sentence,
            "lower" => Case::Lower,
            "upper" => Case::Upper,
            _ => return Err(ParseCaseError::InvalidFormat),
        };

        Ok(case)
    }
}

struct Phrase {
    adj: Word,
    adv: Word,
    noun: Word,
    format: Case,
}

impl Phrase {
    fn with_case(mut self, f: Case) -> Self {
        self.format = f;
        self
    }
}

impl FromStr for Phrase {
    type Err = ParsePhraseError;

    fn from_str(sha: &str) -> ShaResult<Phrase> {
        // Ensure that the sha is at least 8 characters so that
        // when we extract the first 8 there is something there.
        let sha = format!("{:0>8}", sha);
        let adv = lookup(sha[0..3].parse()?, &dictionary::adverbs::WORDS);
        let adj = lookup(sha[3..5].parse()?, &dictionary::adjectives::WORDS);
        let noun = lookup(sha[5..8].parse()?, &dictionary::nouns::WORDS);

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

fn main() {
    let matches = app_matches();

    let format = if let Some(fmt) = matches.value_of("format") {
        fmt.parse().expect("Invalid format specified")
    } else {
        Case::Lower
    };

    if let Some(shas) = matches.values_of("SHA") {
        shas.for_each(|sha| {
            println!("{}", &sha.parse::<Phrase>().unwrap().with_case(format))
        });
    } else if atty::is(Stream::Stdin) {
        from_random_sha(format)
    } else {
        // no args, check stdin
        from_stdin(format);
    };
}

const FORMAT_OPTIONS: [&'static str; 8] = [
    "snake",
    "kebab",
    "camel",
    "pascal",
    "title",
    "sentence",
    "upper",
    "lower",
];

fn app_matches() -> ArgMatches<'static> {
    App::new("Git Release Names")
        .author("Kevin Choubacha <chewbacha@gmail.com>")
        .about(
            "Takes a git sha and uses it's relatively unique combination of letters and number \
                to generate a release name",
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .takes_value(true)
                .possible_values(&FORMAT_OPTIONS)
                .alias("f")
                .help("Declares the return format of the phrase."),
        )
        .arg(Arg::with_name("SHA").multiple(true).help(
            "Each arg should be a sha. If they are less than 8 characters they will be padded",
        ))
        .get_matches()
}

fn from_random_sha(format: Case) {
    println!(
        "{}",
        &format!("{:8x}", rand::random::<usize>())
            .parse::<Phrase>()
            .unwrap()
            .with_case(format)
    );
}

fn from_stdin(format: Case) {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(size) if size > 0 => {
                println!(
                    "{}",
                    &line.trim().parse::<Phrase>().unwrap().with_case(format)
                )
            }
            _ => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_into_a_word() {
        let word = Word::from_str("a").unwrap();
        assert_eq!(word.hash, 10);
    }

    #[test]
    fn it_can_detect_non_hex_chars() {
        let result = Word::from_str("z");
        assert!(result.is_err());
    }

    #[test]
    fn it_can_look_up_from_a_dictionary() {
        let word = lookup(Word::from_str("a").unwrap(), &dictionary::adverbs::WORDS);
        assert_eq!(word.word, Some("proximally".to_string()));
    }

    #[test]
    fn it_will_overflow_the_dictionary_index() {
        let word = lookup("a".parse().unwrap(), &["hello"]);
        assert_eq!(word.word, Some("hello".to_string()));
    }

    #[test]
    fn it_can_format_the_word() {
        let word = Word::from_str("a").unwrap();
        assert_eq!("", format!("{}", word));
        let word = lookup(word, &dictionary::adverbs::WORDS);
        assert_eq!("proximally", format!("{}", word));
    }

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

    #[test]
    fn str_can_be_parsed_to_a_format() {
        assert_eq!(Case::Snake, "snake".parse::<Case>().unwrap());
        assert_eq!(Case::Kebab, "kebab".parse::<Case>().unwrap());
        assert_eq!(Case::Camel, "camel".parse::<Case>().unwrap());
        assert_eq!(Case::Pascal, "pascal".parse::<Case>().unwrap());
        assert_eq!(Case::Title, "title".parse::<Case>().unwrap());
        assert_eq!(Case::Sentence, "sentence".parse::<Case>().unwrap());
        assert_eq!(Case::Lower, "lower".parse::<Case>().unwrap());
        assert_eq!(Case::Upper, "upper".parse::<Case>().unwrap());
        assert!("alsdkfj".parse::<Case>().is_err());
    }
}
