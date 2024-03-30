// use std::io::{BufRead, BufReader};
// use std::include_str;
use std::io::prelude::*;
use std::fmt;
use clap::Parser;
// use anyhow::{Context, Result};
use anyhow::{Result};

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
    

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

fn mersenne31_fold(acc:u64, suc:u64) -> u64 {
    let mut result = acc * suc;
    result ^= result << 31;
    result &= 0x7fffffff;
    result
}

fn hash_anagram(word:&str) -> u64 {
    let mut acc:u64 = 1;
    for c in word.chars() {
        acc = mersenne31_fold(acc, c as u64);
    }
    acc + word.len() as u64
    // let hash:u64 = word.len() as u64;
    // hash
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let hash_match = hash_anagram(&args.letters);
    let letters = Word12::new(&args.letters);
    let filename = "WOW24.txt";
    // Approach #1 - include wordlist in crate
    // let wow = include_str!(filename);
    // let f = BufReader::new(wow);
    // let f = BufReader::new(std::fs::File::open(filename).expect("open failed"));
    // for word in f.lines() {
        // match word {
            // Err(why) => panic!("couldn't read: {}", why),
            // Ok(word) => println!("{:}", &word),
        // }
    // }

    // Approach #2 - fast and simple
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            // println!("{}", line);
            if hash_match == hash_anagram(&line) {
                println!("{}", line);
            }
        }
    }

    // /// Approach #3 - slow and simple
    // let content = std::fs::read_to_string(&filename)
        // .with_context(|| format!("could not read file `{}`", filename))?;

    // for line in content.lines() {
        // if hash_match == hash_anagram(line) {
            // println!("{}", line);
        // }
    // }

    println!("letters {:?}", args.letters);
    println!("{}", letters);
    println!("{:?}", letters);

    // let v:Vec<u8> = (0..12).map(|ii| (((A>>(5*ii)) & 31) + 64) as u8)
        // .take_while(|x| *x > 64 )
        // .collect(); 
    // let s = String::from_utf8(v).expect("Found invalid UTF-8");
    // let n = &args.letters.length();
    // let ss = (0..n).map(|ii| (((A>>(5*ii)) & 31) + 64) as u8)
        // .take_while(|x| *x > 64 )
        // .collect::<Vec<u8>>().as_str();
    Ok(())
}
