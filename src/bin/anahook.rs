use std::io::{ BufRead, BufReader };
use clap::Parser;
use itertools::Itertools;

use anagram_wow::char_set::CharSet;
use anagram_wow::char_multiset::CharMultiSet;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Letters to use
    letters: String,
}

fn main() {
    let args = Args::parse();

    let required = CharSet::from(args.letters.as_str());
    let target = CharMultiSet::from(args.letters.to_uppercase().as_str());
    let length = args.letters.len();
    const WORDLIST:&str = include_str!("../share/WOW24.txt");

    let matches = BufReader::new(WORDLIST.as_bytes())
        .lines()
        .flatten()
        .filter(|s| length+1  == s.len())
        .filter(|s| CharSet::from(s.as_str()).contains(&required))
        .filter(|s| CharMultiSet::from(s.as_str()).contains(&target))
        .join("\n")
        ;

    println!("{}", matches);
}
