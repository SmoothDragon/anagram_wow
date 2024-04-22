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

#[derive(Debug)]
struct Query {
    required_set: CharSet,
    required: CharMultiSet,
    allowed_set: CharSet,
    allowed: CharMultiSet,
    length_min: usize,
    length_max: usize,
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
            required: CharMultiSet::from(&*required),
            allowed_set: CharSet::from(&*allowed),
            allowed: CharMultiSet::from(&*allowed),
            length_min: required.len(),
            length_max: allowed.len()+blanks,
            blanks: blanks,
        }
    }
}

fn anagram(query: Query, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    seq.into_iter()
        .filter(move |s| (query.length_min..=query.length_max).contains(&(s.as_ref().len())))
        .filter(move |s| CharSet::from(&*s.as_ref()).contains(query.required_set))
        .filter(move |s| CharMultiSet::from(&*s.as_ref()).contains(query.required))
        .filter(move |s| query.allowed.blanks_needed(CharMultiSet::from(&*s.as_ref())) <= query.blanks)
}

// TODO: Should this be deleted?
fn anahook(query: Query, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    seq.into_iter()
        .filter(move |s| query.length_max+1 == (s.as_ref().len()))
        .filter(move |s| CharSet::from(&*s.as_ref()).contains(query.allowed_set))
        .filter(move |s| CharMultiSet::from(&*s.as_ref()).contains(query.allowed))
        .filter(move |s| query.allowed.blanks_needed(CharMultiSet::from(&*s.as_ref())) == 1+query.blanks)
}

fn letterbank(query: Query, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    seq.into_iter()
        .filter(move |s| query.length_min <= s.as_ref().len())
        .filter(move |s| CharSet::from(&*s.as_ref()).contains(query.required_set))
        .filter(move |s| query.allowed_set.blanks_needed(CharSet::from(&*s.as_ref())) <= query.blanks)
}

// fn letterbank(letters: &str, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    // TODO import these
    // let arg_min = 2;
    // let arg_max = 15;

    // let required = CharSet::from(letters.chars()
        // .filter(|ch| ch.is_ascii_uppercase())
        // .collect::<String>()
        // .as_str()
        // );
    // let allowed = CharSet::from(letters.to_uppercase().as_str());
    // let min_length = max(required.len(), arg_min);
    // seq.into_iter()
        // .filter(move |s| (min_length..=arg_max).contains(&(s.as_ref().len())))
        // .filter(move |s| CharSet::from(&*s.as_ref()).contains(required))
        // .filter(move |s| allowed.contains(CharSet::from(&*s.as_ref())))
// }


fn main() {
    let args = Args::parse();

    let word_list = BufReader::new(WORDLIST.as_bytes()).lines().flatten();
    let query = Query::from(args.letters.as_str());

    let ss = match args.method {
        Method::anagram => anagram(query, word_list).join("\n"),
        Method::anahook => anahook(query, word_list).join("\n"),
        Method::letterbank => letterbank(query, word_list).join("\n"),
        _ => "Undefined".to_string(),
    };

    println!("{}", ss);
}
