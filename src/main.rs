extern crate rand;

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
            hash: usize::from_str_radix(&value, 16).expect("Value cannot be translated to a word index"),
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
    let sha = format!("{:8x}", rand::random::<usize>());

    let adv = Word::new(&sha[0..3]).use_dictionary(&adverbs::WORDS);
    let adj = Word::new(&sha[3..5]).use_dictionary(&adjectives::WORDS);
    let n   = Word::new(&sha[5..8]).use_dictionary(&nouns::WORDS);

    println!("adv: {:?}", adv);
    println!("adj: {:?}", adj);
    println!("n:   {:?}", n);
    println!("{} {} {}",
             adv.word.unwrap(),
             adj.word.unwrap(),
             n.word.unwrap())
}
