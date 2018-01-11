extern crate rand;
extern crate atty;
extern crate clap;

use std::fmt::{Display, Formatter, Error};
use std::io::{self, BufRead};
use atty::Stream;
use clap::{Arg, App, ArgMatches};
use std::str::FromStr;

mod adverbs;
mod nouns;
mod adjectives;

#[derive(Debug, Eq, PartialEq)]
enum PhraseParseError {
    NonHexadecimalCharacters,
}

type ShaResult<T> = Result<T, PhraseParseError>;

#[derive(Debug)]
struct Word {
    sha: String,
    hash: usize,
    word: Option<String>,
}

fn lookup(word: Word, dict: &[&str]) -> Word {
    Word {
        word: dict.get(word.hash).map(|s| s.to_string()),
        ..word
    }
}

impl FromStr for Word {
    type Err = PhraseParseError;

    fn from_str(sha: &str) -> ShaResult<Word> {
        if let Ok(hash) = usize::from_str_radix(&sha, 16) {
            Ok(Word {
                sha: sha.to_string(),
                hash,
                word: None,
            })
        } else {
            Err(PhraseParseError::NonHexadecimalCharacters)
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

struct Phrase {
    adj: Word,
    adv: Word,
    noun: Word,
}

impl FromStr for Phrase {
    type Err = PhraseParseError;

    fn from_str(sha: &str) -> ShaResult<Phrase> {
        // Ensure that the sha is at least 8 characters so that
        // when we extract the first 8 there is something there.
        let sha = format!("{:0>8}", sha);
        let adv = lookup(sha[0..3].parse()?, &adverbs::WORDS);
        let adj = lookup(sha[3..5].parse()?, &adjectives::WORDS);
        let noun = lookup(sha[5..8].parse()?, &nouns::WORDS);

        Ok(Phrase { adv, adj, noun })
    }
}

impl Display for Phrase {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {} {}", self.adv, self.adj, self.noun)
    }
}

fn main() {
    let matches = app_matches();

    if let Some(shas) = matches.values_of("SHA") {
        shas.for_each(|sha| println!("{}", &sha.parse::<Phrase>().unwrap()));
    } else if atty::is(Stream::Stdin) {
        from_random_sha()
    } else {
        // no args, check stdin
        from_stdin();
    };
}

fn app_matches() -> ArgMatches<'static> {
    App::new("Git Release Names")
        .author("Kevin Choubacha <chewbacha@gmail.com>")
        .about(
            "Takes a git sha and uses it's relatively unique combination of letters and number \
                to generate a release name",
        )
        .arg(Arg::with_name("SHA").multiple(true).help(
            "Each arg should be a sha. If they are less than 8 characters they will be padded",
        ))
        .get_matches()
}

fn from_random_sha() {
    println!(
        "{}",
        &format!("{:8x}", rand::random::<usize>())
            .parse::<Phrase>()
            .unwrap()
    );
}

fn from_stdin() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(size) if size > 0 => println!("{}", &line.trim().parse::<Phrase>().unwrap()),
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
        let word = lookup(Word::from_str("a").unwrap(), &adverbs::WORDS);
        assert_eq!(word.word, Some("proximally".to_string()));
    }

    #[test]
    fn it_can_format_the_word() {
        let word = Word::from_str("a").unwrap();
        assert_eq!("", format!("{}", word));
        let word = lookup(word, &adverbs::WORDS);
        assert_eq!("proximally", format!("{}", word));
    }

    #[test]
    fn a_phrase_can_be_generated_from_a_str() {
        let phrase = "0a00a00a".parse::<Phrase>().expect("Invalid phrase");
        assert_eq!("immeasurably endways borings", format!("{}", phrase));
    }
}
