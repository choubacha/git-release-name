extern crate rand;
use std::env;

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

    fn use_dictionary(self, dict: &[&str]) -> Word {
        Word { word: dict.get(self.hash).map(|s| s.to_string()), ..self }
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

    let adv = Word::new(&sha[0..3]).use_dictionary(&adverbs::WORDS);
    let adj = Word::new(&sha[3..5]).use_dictionary(&adjectives::WORDS);
    let n   = Word::new(&sha[5..8]).use_dictionary(&nouns::WORDS);

    println!("{} {} {}",
             adv.word.unwrap(),
             adj.word.unwrap(),
             n.word.unwrap())
}
