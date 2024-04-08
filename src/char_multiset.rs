// TODO: Put description here for doc

use std::fmt;
use itertools::Itertools;

#[derive(PartialEq)]
pub struct CharMultiSet(u128);

impl From<&str> for CharMultiSet {
    /// Maps A..Z to a bit in position 1..=26
    /// The AND and SHIFT will work for all u8, even non-alphabeticals.
    /// Upper and lower case letters both map to the same position.
    fn from(word: &str) -> Self {
        Self(word.bytes().fold(0, |acc, ch| (acc + (1<<(((ch as u128)& 0x1f) << 2)))))
    }
}

impl CharMultiSet {
    pub fn contains(&self, other: &CharMultiSet) -> bool {
        ((self.0.wrapping_sub(other.0)) & 0x8888888888888888u128) == 0
    }

    pub fn len(&self) -> usize {
        let mask1 = 0xf0f0f0f0f0f0f0fu64;
        let result = (self.0 + (self.0>>64)) as u64;
        // let result = result.wrapping_mul(0x1111111111111111u64);
        // println!("{}", Self(result as u128));
        // (result >> 56) as usize
        let result = ((result >> 4) & mask1) + (result & mask1) ;
        let result = (result >> 8) + result;
        let result = (result >> 16) + result;
        let result = (result >> 32) + result;
        (result & 0xff) as usize
    }
}

impl fmt::Debug for CharMultiSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = (1..=26)
            .map(|ii| format!("{}", ((self.0 >> (ii<<2)) & 0xf)))
            .join(" ")
            ;
        write!(f, "{}", s)
    }
}
    
impl fmt::Display for CharMultiSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = (1..=26)
            .map(|ii| format!("{}", ((self.0 >> (ii<<2)) & 0xf)))
            .join(" ")
            ;
        write!(f, "{}", s)
    }
}
    

#[cfg(test)]
mod test {
    use super::CharMultiSet;

    #[test]
    fn test_from() {
        println!("{}", CharMultiSet::from("A"));
        println!("{}", CharMultiSet::from("abc"));
        assert_eq!(CharMultiSet(0u128), CharMultiSet::from(""));
        assert_eq!(CharMultiSet(0x30u128), CharMultiSet::from("aAa"));
        assert_eq!(CharMultiSet(0x1110u128), CharMultiSet::from("CAB"));
        assert_eq!(CharMultiSet(0x1110u128), CharMultiSet::from("abc"));
    }

    #[test]
    fn test_contains() {
        assert!(CharMultiSet(0x7u128).contains(&CharMultiSet(0x1u128)));
        assert!(CharMultiSet(0x20u128).contains(&CharMultiSet(0x10u128)));
        assert!(!CharMultiSet(0x10u128).contains(&CharMultiSet(0x20u128)));
        assert!(CharMultiSet::from("RETAIN").contains(&CharMultiSet::from("RAIN")));
        assert!(!CharMultiSet::from("RAIN").contains(&CharMultiSet::from("RETAIN")));
    }

    #[test]
    fn test_len() {
        println!("{}", CharMultiSet::from("ZESTIER"));
        assert_eq!(CharMultiSet::from("ZESTIER").len(), 7);
    }
}

