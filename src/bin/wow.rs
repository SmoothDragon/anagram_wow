use std::io::{ BufRead, BufReader };
use std::fmt::Display;
use clap::*;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

use anagram_wow::{CharSet, CharMultiSet, Query};

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
   boxed,
}

fn anagram(query: Query, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    seq.into_iter()
        .filter(move |s| (query.length_min..=query.length_max).contains(&(s.as_ref().len())))
        .filter(move |s| CharSet::from(s.as_ref()).contains(query.required_set))
        .filter(move |s| CharMultiSet::from(s.as_ref()).contains(query.required))
        .filter(move |s| query.allowed.blanks_needed(CharMultiSet::from(s.as_ref())) <= query.blanks)
}

fn anahook(query: Query, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    seq.into_iter()
        .filter(move |s| query.length_max+1 == (s.as_ref().len()))
        .filter(move |s| CharSet::from(s.as_ref()).contains(query.allowed_set))
        .filter(move |s| CharMultiSet::from(s.as_ref()).contains(query.allowed))
        .filter(move |s| query.allowed.blanks_needed(CharMultiSet::from(s.as_ref())) == 1+query.blanks)
}

fn letterbank(query: Query, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    seq.into_iter()
        .filter(move |s| query.length_min <= s.as_ref().len())
        .filter(move |s| CharSet::from(s.as_ref()).contains(query.required_set))
        .filter(move |s| query.blanks!=0 || query.allowed_set.contains(CharSet::from(s.as_ref()))) 
        .filter(move |s| query.blanks==0 || query.allowed_set.blanks_needed(CharSet::from(s.as_ref())) <= query.blanks)
}

fn boxed_adjacent(letters: &str, word: &str) -> bool {
    let mut last_pos = usize::MAX;
    let letters = letters.to_uppercase();
    for ch in word.to_uppercase().chars() {
        let pos = letters.find(ch).unwrap() / 3;
        // println!("letters: {}, word: {}, ch: {}, pos: {}", letters, word, ch, pos);
        if pos == last_pos { return false };
        last_pos = pos;
    }
    true
}


fn boxed(letters: String, seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> impl Iterator<Item=impl AsRef<str>+Display> {
    let query = Query::from(&*letters);
    seq.into_iter()
        .filter(move |s| CharSet::from(s.as_ref()).contains(query.required_set))
        .filter(move |s| query.blanks!=0 || query.allowed_set.contains(CharSet::from(s.as_ref()))) 
        .filter(move |s| query.blanks==0 || query.allowed_set.blanks_needed(CharSet::from(s.as_ref())) <= query.blanks)
        .filter(move |s| boxed_adjacent(&letters, s.as_ref()))
}

fn main() {
    let args = Args::parse();

    let letters = args.letters.to_string();
    let word_list = BufReader::new(WORDLIST.as_bytes()).lines().map_while(Result::ok);
    let query = Query::from(&*letters);

    let ss = match args.method {
        Method::anagram => anagram(query, word_list).join("\n"),
        Method::anahook => anahook(query, word_list).join("\n"),
        Method::letterbank => letterbank(query, word_list).join("\n"),
        Method::boxed => boxed(letters, word_list).join("\n"),
        _ => "Undefined".to_string(),
    };

    println!("{}", ss);
}
