extern crate rand;
use std::env;
use std::fmt::{Display, Formatter, Error};
use std::io::{self, Read};

#[derive(Debug)]
struct Word {
    sha: String,
    hash: usize,
    word: Option<String>,
}

impl Word {
    fn new(sha: &str) -> Word {
        Word {
            sha: sha.to_string(),
            hash: usize::from_str_radix(&sha, 16).expect("Sha is not valid hex"),
            word: None,
        }
    }

    fn lookup(self, dict: &[&str]) -> Word {
        Word { word: dict.get(self.hash).map(|s| s.to_string()), ..self }
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

mod adverbs;
mod nouns;
mod adjectives;

struct Phrase {
    adj: Word,
    adv: Word,
    noun: Word,
}

impl Phrase {
    fn new(sha: &str) -> Phrase {
        // Ensure that the sha is at least 8 characters so that
        // when we extract the first 8 there is something there.
        let sha = format!("{:0>8}", sha);
        let adv  = Word::new(&sha[0..3]).lookup(&adverbs::WORDS);
        let adj  = Word::new(&sha[3..5]).lookup(&adjectives::WORDS);
        let noun = Word::new(&sha[5..8]).lookup(&nouns::WORDS);

        Phrase { adv, adj, noun }
    }
}

impl Display for Phrase {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {} {}", self.adv, self.adj, self.noun)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sha = if args.len() > 1 {
        args[1].to_owned()
    } else {
        // no args, check stdin
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer);
        if buffer.len() > 0 {
            buffer
        } else {
            format!("{:8x}", rand::random::<usize>())
        }
    };
    println!("{}", Phrase::new(&sha));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_into_a_word() {
        let word = Word::new("a");
        assert_eq!(word.hash, 10);
    }

    #[test]
    fn it_can_look_up_from_a_dictionary() {
        let word = Word::new("a").lookup(&adverbs::WORDS);
        assert_eq!(word.word, Some("proximally".to_string()));
    }

    #[test]
    fn it_can_format_the_word() {
        let word = Word::new("a");
        assert_eq!("", format!("{}", word));
        let word = word.lookup(&adverbs::WORDS);
        assert_eq!("proximally", format!("{}", word));
    }

    #[test]
    fn a_phrase_can_be_generated() {
        let phrase = Phrase::new("0a00a00a");
        assert_eq!("immeasurably endways borings", format!("{}", phrase));
    }
}
