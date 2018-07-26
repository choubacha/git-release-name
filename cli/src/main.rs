extern crate atty;
extern crate clap;
extern crate git_release_name;
extern crate rand;

use atty::Stream;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::{self, BufRead};

use git_release_name::Case;

fn main() {
    let matches = app_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        list::list_dictionary(matches);
    } else {
        let format = if let Some(fmt) = matches.value_of("format") {
            fmt.parse().expect("Invalid format specified")
        } else {
            Case::Lower
        };

        if let Some(shas) = matches.values_of("SHA") {
            shas.for_each(|sha| {
                println!(
                    "{}",
                    git_release_name::lookup(&sha)
                        .expect("Invalid sha")
                        .with_case(format)
                )
            });
        } else if atty::is(Stream::Stdin) {
            from_random_sha(format)
        } else {
            // no args, check stdin
            from_stdin(format);
        };
    }
}

const FORMAT_OPTIONS: [&'static str; 8] = [
    "snake", "kebab", "camel", "pascal", "title", "sentence", "upper", "lower",
];

fn app_matches() -> ArgMatches<'static> {
    App::new("Git Release Names")
        .author("Kevin Choubacha <chewbacha@gmail.com>")
        .about(
            "Takes a git sha and uses it's relatively unique combination of letters and number \
             to generate a release name",
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List out the dictionary words that are in use.")
                .arg(
                    Arg::with_name("includes")
                        .long("include")
                        .short("i")
                        .takes_value(true)
                        .possible_values(&["nouns", "n", "adjectives", "adj", "adverbs", "adv"])
                        .multiple(true)
                        .help("Specify which types of words to list."),
                )
                .arg(
                    Arg::with_name("format")
                        .long("format")
                        .short("f")
                        .takes_value(true)
                        .possible_values(&["csv", "fixed"])
                        .help("Specify the row format to use"),
                ),
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
        .arg(Arg::with_name("bench").long("bench"))
        .arg(Arg::with_name("SHA").multiple(true).help(
            "Each arg should be a sha. If they are less than 8 characters they will be padded",
        ))
        .get_matches()
}

mod list {
    use clap::ArgMatches;
    use git_release_name::{list, Entry, Kind};

    struct List {
        n: bool,
        adv: bool,
        adj: bool,
    }

    impl List {
        fn new() -> List {
            List {
                n: false,
                adv: false,
                adj: false,
            }
        }

        fn apply(&mut self, val: &str) {
            match val {
                "n" | "nouns" => self.n = true,
                "adv" | "adverbs" => self.adv = true,
                "adj" | "adjectives" => self.adj = true,
                _ => {}
            }
        }

        fn entries(self) -> Vec<Entry> {
            let mut entries = Vec::new();
            if self.n {
                entries.append(&mut list(Kind::Noun));
            }
            if self.adv {
                entries.append(&mut list(Kind::Adv));
            }
            if self.adj {
                entries.append(&mut list(Kind::Adj));
            }
            if !(self.n || self.adv || self.adj) {
                entries.append(&mut list(Kind::Noun));
                entries.append(&mut list(Kind::Adv));
                entries.append(&mut list(Kind::Adj));
            }
            entries
        }
    }

    pub fn list_dictionary(matches: &ArgMatches) {
        let mut list = List::new();
        if let Some(includes) = matches.values_of("includes") {
            for val in includes {
                list.apply(val)
            }
        }
        let entries = list.entries();

        match matches.value_of("format") {
            Some("csv") => print_csv(&entries),
            Some("fixed") => print_fixed(&entries),
            _ => print_fixed(&entries),
        }
    }

    fn print_fixed(entries: &[Entry]) {
        println!(
            "{kind:>4} {word:<20} {index}",
            kind = "type",
            word = "word",
            index = "index"
        );

        for entry in entries {
            println!(
                "{kind:>4} {word:<20} {index}",
                kind = match entry.kind {
                    Kind::Noun => "noun",
                    Kind::Adj => "adj",
                    Kind::Adv => "adv",
                },
                index = entry.index,
                word = entry.word
            )
        }
    }

    fn print_csv(entries: &[Entry]) {
        println!(
            "{kind},{word},{index}",
            kind = "type",
            word = "word",
            index = "index"
        );

        for entry in entries {
            println!(
                "{kind},{word},{index}",
                kind = match entry.kind {
                    Kind::Noun => "noun",
                    Kind::Adj => "adj",
                    Kind::Adv => "adv",
                },
                index = entry.index,
                word = entry.word
            )
        }
    }
}

fn from_random_sha(format: Case) {
    println!(
        "{}",
        git_release_name::lookup(&format!("{:08x}", rand::random::<u32>()))
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
            Ok(size) if size > 0 => println!(
                "{}",
                git_release_name::lookup(&line.trim())
                    .unwrap()
                    .with_case(format)
            ),
            _ => break,
        }
    }
}
