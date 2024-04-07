use std::io::{ BufRead, BufReader };
use std::error::Error;
// use std::include_str;
// use std::io::prelude::*;
use std::fmt;
use clap::Parser;
// use anyhow::{Context, Result};
// use anyhow::{Result};
use std::collections::HashMap;
use itertools::Itertools;

/// Search for words that match the letters given
#[derive(Parser)]
struct Cli {
    /// Letters to use
    letters: String,
}


#[derive(Debug, PartialEq)]
struct Word12(u64);

impl Word12 {
    pub fn new(word: &str) -> Self {
        let mut result:u64 = 0;
        let base:u64 = 'A' as u64;
        for (ii, ch) in word.chars().enumerate() {
            result += ((ch as u64) - base + 1) << (5 * ii);
        }
        result += (word.len() as u64) << 60;
        Self(result)
    }
}

impl fmt::Display for Word12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = (self.0 >> 60) as usize;
        let arr: [u8; 12] = std::array::from_fn::<u8, 12, _>(|i| (((self.0>>(5*i))&31)+64) as u8);
        let s = std::str::from_utf8(&arr[0..n]).expect("invalid utf-8 sequence");
        write!(f, "{}", s)
    }
}
    
#[derive(Debug, PartialEq)]
struct CharPrime(u64);

impl CharPrime {
    pub fn new(word: &str) -> Self {
        let mut conso:u64 = 1;
        let mut vowel:u64 = 0;
        let conso_prime: HashMap<char, u64> = [
            ('S',  2), ('N',  3), ('T',  5), ('R',  7), ('L', 11), ('D', 13), ('C', 17), ('G', 19), ('P', 23),
            ('U', 29), ('M', 31), ('H', 37), ('B', 41), ('Y', 43), ('F', 47), ('Z', 53), ('K', 59), ('W', 61),
            ('V', 67), ('X', 71), ('J', 73), ('Q', 79)
        ].iter().cloned().collect();
        let vowel_shift: HashMap<char, u64> = [ ('A', 1), ('E', 8), ('I', 64), ('O', 512) ].iter().cloned().collect();

        for ch in word.chars() {
            if vowel_shift.contains_key(&ch) {
                vowel += vowel_shift[&ch];
            } else {
                conso *= conso_prime[&ch];
            }
        }
        Self((conso<<12) + vowel)
    }
}

impl fmt::Display for CharPrime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let conso_prime: HashMap<char, u64> = [
            ('S',  2), ('N',  3), ('T',  5), ('R',  7), ('L', 11), ('D', 13), ('C', 17), ('G', 19), ('P', 23),
            ('U', 29), ('M', 31), ('H', 37), ('B', 41), ('Y', 43), ('F', 47), ('Z', 53), ('K', 59), ('W', 61),
            ('V', 67), ('X', 71), ('J', 73), ('Q', 79)
        ].iter().cloned().collect();
        let mut string = String::new();
        let mut hash = self.0 >> 12;
        for (key, value) in conso_prime.iter() {
            while hash % value == 0 {
                string.push(*key);
                hash = hash / value;
            }
        }
        let bottom = self.0 & 0xfff;
        for (ii, ch) in "AEIO".chars().enumerate() {
            let limit = (bottom >> (3*ii)) & 7;
            for _ in 0..limit { string.push(ch); }
        }
        write!(f, "{}", string)
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let letters = Word12::new(&args.letters.to_uppercase());
    let target = CharPrime::new(&args.letters.to_uppercase());
    let length = &args.letters.len();
    const WORDLIST:&str = include_str!("../share/WOW24.txt");

    let reader = BufReader::new(WORDLIST.as_bytes());
    // let filename = "WOW24.txt";
    // let reader = BufReader::new(std::fs::File::open(filename).expect("open failed"));

    let matches = reader
        .lines()
        .flatten()
        .filter(|s| {&s.len()==length})
        .filter(|s| {target==CharPrime::new(&s)})
        .join("\n")
        ;

    println!("{}", matches);
    // for item in matches {
        // println!("FILTERED: {}", item)
    // }




    println!("letters {:?}", args.letters);
    println!("{}", letters);
    println!("{:?}", letters);
    println!("Still in main");

    Ok(())
}
