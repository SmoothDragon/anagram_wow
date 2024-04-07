use std::io::{ BufRead, BufReader };
use std::cmp::*;
use clap::Parser;
use itertools::Itertools;

use anagram_wow::char_set::CharSet;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Letters to use
    letters: String,

    /// Minimum characters of output
    #[arg(short, long, default_value_t = 0)]
    min: usize,

    /// Maximum characters of output
    #[arg(long, default_value_t = 15)]
    max: usize,
}

fn main() {
    let args = Args::parse();

    let required = CharSet::from(args.letters.chars()
        .filter(|ch| ch.is_ascii_uppercase())
        .collect::<String>()
        .as_str()
        );
    let allowed = CharSet::from(args.letters.to_uppercase().as_str());
    let min_length = max(required.len(), args.min);
    const WORDLIST:&str = include_str!("../share/WOW24.txt");

    let matches = BufReader::new(WORDLIST.as_bytes())
        .lines()
        .flatten()
        .filter(|s| (min_length..=args.max).contains(&s.len()))
        .filter(|s| CharSet::from(s.as_str()).contains(&required))
        .filter(|s| allowed.contains(&CharSet::from(s.as_str())))
        .join("\n")
        ;

    println!("{}", matches);
}
