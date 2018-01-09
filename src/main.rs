extern crate rand;
use std::env;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
struct Word {
    value: String,
    hash: usize,
    word: Option<String>,
}

impl Word {
    fn new(value: &str) -> Word {
        Word {
            value: value.to_string(),
            hash: usize::from_str_radix(&value, 16).expect("Sha is not valid hex"),
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let sha = if args.len() > 1 {
        format!("{:0>8}", args[1])
    } else {
        format!("{:8x}", rand::random::<usize>())
    };

    let adv = Word::new(&sha[0..3]).lookup(&adverbs::WORDS);
    let adj = Word::new(&sha[3..5]).lookup(&adjectives::WORDS);
    let n   = Word::new(&sha[5..8]).lookup(&nouns::WORDS);

    println!("{} {} {}", adv, adj, n);
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
}
