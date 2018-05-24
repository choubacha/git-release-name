extern crate atty;
extern crate clap;
extern crate rand;
extern crate git_release_name;

use atty::Stream;
use clap::{App, Arg, ArgMatches};
use std::io::{self, BufRead};

use git_release_name::Case;

fn main() {
    let matches = app_matches();

    let format = if let Some(fmt) = matches.value_of("format") {
        fmt.parse().expect("Invalid format specified")
    } else {
        Case::Lower
    };

    if let Some(shas) = matches.values_of("SHA") {
        shas.for_each(|sha| println!("{}", git_release_name::lookup(&sha).unwrap().with_case(format)));
    } else if atty::is(Stream::Stdin) {
        from_random_sha(format)
    } else {
        // no args, check stdin
        from_stdin(format);
    };
}

const FORMAT_OPTIONS: [&'static str; 8] = [
    "snake", "kebab", "camel", "pascal", "title", "sentence", "upper", "lower"
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
        git_release_name::lookup(&format!("{:8x}", rand::random::<usize>()))
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
