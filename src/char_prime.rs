use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;

const PRIMES: [u64; 26] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,91,97];

lazy_static! {
    static ref CONSO_PRIME: HashMap<char, u64> = "SNTRLDCGPUMHBYFZKWVXJQ".chars().zip(PRIMES).collect();
}

#[derive(Debug, PartialEq)]
pub struct CharPrime(u64);

impl From<&str> for CharPrime {
    fn from(word: &str) -> Self {
        let mut conso:u64 = 1;
        let mut vowel:u64 = 0;
        let vowel_shift: HashMap<char, u64> = [ ('A', 1), ('E', 8), ('I', 64), ('O', 512) ].iter().cloned().collect();

        for ch in word.chars() {
            if vowel_shift.contains_key(&ch) {
                vowel += vowel_shift[&ch];
            } else {
                // conso *= conso_prime[&ch];
                conso *= CONSO_PRIME[&ch];
            }
        }
        Self((conso<<12) + vowel)
    }
}


impl fmt::Display for CharPrime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        let mut hash = self.0 >> 12;
        for (key, value) in CONSO_PRIME.iter() {
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

#[cfg(test)]
mod test {
    use super::CharPrime;

    #[test]
    fn basics() {
        assert_eq!(CharPrime(4096u64), CharPrime::new(""));
        assert_eq!(CharPrime::new("ANESTRI"), CharPrime::new("RETAINS"));
        // TODO: format output is random due to HashMap
        // assert_eq!(format!("{}",CharPrime::new("ANESTRI")), "RTNSAEI".to_string());
    }
}

// impl CharPrime {
    // pub fn new(word: &str) -> Self {
        // let mut conso:u64 = 1;
        // let mut vowel:u64 = 0;
        // let vowel_shift: HashMap<char, u64> = [ ('A', 1), ('E', 8), ('I', 64), ('O', 512) ].iter().cloned().collect();

        // for ch in word.chars() {
            // if vowel_shift.contains_key(&ch) {
                // vowel += vowel_shift[&ch];
            // } else {
                // conso *= CONSO_PRIME[&ch];
            // }
        // }
        // Self((conso<<12) + vowel)
    // }
// }

