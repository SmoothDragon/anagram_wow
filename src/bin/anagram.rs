use std::io::{ BufRead, BufReader };
use clap::Parser;
use itertools::Itertools;

use anagram_wow::char_set::CharSet;
// use anagram_wow::char_prime::CharPrime;
use anagram_wow::char_multiset::CharMultiSet;


/// Search for words that match the letters given
#[derive(Parser)]
struct Cli {
    /// Letters to use
    letters: String,
}


fn main() {
    let args = Cli::parse();

    // If numbers are separated, they will be joined into a multidigit number.
    let blanks = args.letters.chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .fold(0, |acc, d| 10*acc+d)
        ;
    let letters = args.letters.to_uppercase().chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .join("")
        ;
    println!("{}", blanks);
    println!("{}", letters);
    // let target = CharPrime::from(letters.as_str());
    let target_set = CharSet::from(letters.as_str());
    let target = CharMultiSet::from(letters.as_str());
    let length = letters.len() + blanks;
    println!("{}", length);
    const WORDLIST:&str = include_str!("../share/WOW24.txt");

    let matches = BufReader::new(WORDLIST.as_bytes())
        .lines()
        .flatten()
        .filter(|s| {s.len()==length})
        .filter(|s| CharSet::from(s.as_str()).contains(target_set))
        // .filter(|s| {target==CharPrime::from(s.as_str())})
        .filter(|s| CharMultiSet::from(s.as_str()).contains(target))
        .join("\n")
        ;

    println!("{}", matches);
}
