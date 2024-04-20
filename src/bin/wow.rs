use std::io::{ BufRead, BufReader };
use std::fmt::Display;
use clap::*;
use itertools::Itertools;

use anagram_wow::{CharSet, CharMultiSet};

const WORDLIST:&str = include_str!("../share/WOW24.txt");

/// Search for words that match the letters given

#[derive(clap::Parser)]
struct Args {
    #[clap(value_enum)]
    method: Method,
    letters: String,
}

#[derive(clap::ValueEnum, Clone)]
#[allow(non_camel_case_types)]
enum Method {
   anagram,
   anahook,
   letterbank,
   hooks,
   define,
}

fn anagram(letters: &str, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    let target_set = CharSet::from(letters);
    let target = CharMultiSet::from(letters);
    let length = target.len();
    seq.into_iter()
        .filter(move |s| (*s.as_ref()).len()==length )
        .filter(move |s| CharSet::from(&*s.as_ref()).contains(target_set))
        .filter(move |s| CharMultiSet::from(&*s.as_ref()).contains(target))
}

fn anahook(letters: &str, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    let target_set = CharSet::from(letters);
    let target = CharMultiSet::from(letters);
    let length = target.len();
    seq.into_iter()
        .filter(move |s| (*s.as_ref()).len()==length+1 )
        .filter(move |s| CharSet::from(&*s.as_ref()).contains(target_set))
        .filter(move |s| CharMultiSet::from(&*s.as_ref()).contains(target))
}



fn main() {
    let args = Args::parse();

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
    let word_list = BufReader::new(WORDLIST.as_bytes()).lines().flatten();
    // let target_set = CharSet::from(&*letters);
    // let target = CharMultiSet::from(letters.as_str());
    // let length = letters.len() + blanks;

    let ss = match args.method {
        Method::anagram => anagram(&letters, word_list).join("\n"),
        Method::anahook => anahook(&letters, word_list).join("\n"),
        _ => "Undefined".to_string(),
    };
    println!("{}", ss);

}
