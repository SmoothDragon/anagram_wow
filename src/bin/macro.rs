/*
use std::io::{ BufRead, BufReader };
use std::fmt::Display;
use clap::*;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use fasthash::city;

use anagram_wow::{CharSet, CharMultiSet, Query};
const WORDLIST:&str = include_str!("../share/WOW24.txt");
const WORDBYTE:&[u8] = include_bytes!("../share/WOW24.txt");
// const WORDARRAY:[&u8] = BufReader::new(WORDLIST.as_bytes()).lines().flatten();
fn wordmap(seq: impl IntoIterator<Item=impl AsRef<str>+Display>) -> BTreeMap<u32, CharSet> {
    seq.into_iter()
        .filter(move |s| 3 == s.as_ref().len())
        .map(move |s| (city::hash32(&*s.as_ref()), CharSet::from(&*s.as_ref())) )
        .collect::<BTreeMap<u32, CharSet>>()
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        { $a+$b }
    }
}

macro_rules! const_array {
    ($a:expr) => {
        $a.split("\r\n")
            .filter(move |s| 2 == s.len())
            .map(move |s| (city::hash32(s), CharSet::from(s)) )
            .collect::<BTreeMap<u32, CharSet>>()
    }
}

fn main() {
    let a:usize =  add!(1,2);
    // let b = const_array!("abc\ndefg\nxyz");
    const b:BTreeMap<u32, CharSet> = const_array!(WORDLIST);
    println!("{}", a);
    println!("{:?}", b);
    // for word in WORDLIST.split("\r\n") {
        // println!("{:?}", word);
    // }
    // const wm:BTreeMap<u32, CharSet> =  wordmap(WORDLIST.split("\r\n"));
    // let keys: Vec<CharMultiSet> = wm.into_keys().collect();
    // println!("{:?}", wm);
    // for (key, value) in wm.into_iter() {
        // println!("{}:{}", key, value);
    // }
}
*/
