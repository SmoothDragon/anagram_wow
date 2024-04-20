use std::io::{ BufRead, BufReader };
use std::fmt::Display;
use core::cmp::max;
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

struct Query {
    required_set: CharSet,
    // required_multiset: CharMultiSet,
    allowed_set: CharSet,
    // lenth_min: usize,
    // lenth_max: usize,
    blanks: usize,
}

impl From<&str> for Query {
    /// Parse an alphanumeric string
    /// Uppercase letters are required
    /// Lowercase letters are optional
    /// Number represents # of blanks allowed
    fn from(word: &str) -> Self {
        // if !word.chars().all(char::alphanumeric) { return None };
        let blanks = word.chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .fold(0, |acc, d| 10*acc+d)
            ;
        let required = word.chars()
            .filter(|ch| ch.is_ascii_uppercase())
            .join("")
            ;
        let allowed = word.to_uppercase().chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .join("")
            ;
        Self{
            required_set: CharSet::from(&*required),
            allowed_set: CharSet::from(&*allowed),
            blanks: blanks,
        }
    }
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

fn letterbank(letters: &str, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    // TODO import these
    let arg_min = 2;
    let arg_max = 15;

    let required = CharSet::from(letters.chars()
        .filter(|ch| ch.is_ascii_uppercase())
        .collect::<String>()
        .as_str()
        );
    let allowed = CharSet::from(letters.to_uppercase().as_str());
    let min_length = max(required.len(), arg_min);
    seq.into_iter()
        .filter(move |s| (min_length..=arg_max).contains(&(s.as_ref().len())))
        .filter(move |s| CharSet::from(&*s.as_ref()).contains(required))
        .filter(move |s| allowed.contains(CharSet::from(&*s.as_ref())))
}



fn main() {
    let args = Args::parse();

    // TODO: make a Query struct
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
        Method::letterbank => letterbank(&args.letters, word_list).join("\n"),
        _ => "Undefined".to_string(),
    };
    println!("{}", ss);

}
