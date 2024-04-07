use std::io::{ BufRead, BufReader };
use clap::Parser;
use itertools::Itertools;

use anagram_wow::char_set::CharSet;
use anagram_wow::char_prime::CharPrime;


/// Search for words that match the letters given
#[derive(Parser)]
struct Cli {
    /// Letters to use
    letters: String,
}


fn main() {
    let args = Cli::parse();

    let target = CharPrime::from(args.letters.to_uppercase().as_str());
    let target_set = CharSet::from(args.letters.to_uppercase().as_str());
    let length = args.letters.len();
    const WORDLIST:&str = include_str!("../share/WOW24.txt");

    let matches = BufReader::new(WORDLIST.as_bytes())
        .lines()
        .flatten()
        .filter(|s| {s.len()==length})
        .filter(|s| {target_set==CharSet::from(s.as_str())})
        .filter(|s| {target==CharPrime::from(s.as_str())})
        .join("\n")
        ;

    println!("{}", matches);
}
