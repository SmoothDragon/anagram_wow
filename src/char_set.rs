// TODO: Put description here for doc

use std::fmt;
use itertools::Itertools;
use bitintr::Popcnt;

#[derive(PartialEq, Clone, Copy)]
pub struct CharSet(u32);

impl From<&str> for CharSet {
    /// Maps A..Z to a bit in position 1..=26
    /// The AND and SHIFT will work for all u8, even non-alphabeticals.
    /// Upper and lower case letters both map to the same position.
    fn from(word: &str) -> Self {
        Self(word.bytes().fold(0, |acc, ch| (acc | (1<<(ch & 0x1f)))))
    }
}

impl CharSet {
    pub fn contains(self, other: CharSet) -> bool {
        other.0 | self.0 == self.0
    }

    pub fn len(self) -> usize {
        (self.0).popcnt() as usize
    }

    pub fn blanks_needed(self, other: CharSet) -> usize {
        ((self.0 | other.0) - self.0).popcnt() as usize
    }

}

impl fmt::Debug for CharSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = ('A'..='Z').enumerate()
            .map(|(ii, ch)| format!("{}{}", ch, ((self.0 >> (ii+1)) & 0x1) ))
            .join(" ")
            ;
        write!(f, "{}", s)
    }
}
    
impl fmt::Display for CharSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = (1..=26)
            .map(|ii| format!("{}", ((self.0 >> ii) & 0x1)))
            .join(" ")
            ;
        write!(f, "{}", s)
    }
}
    
//
// impl Into<CharSet> for &String {
    // /// Maps A..Z to a bit in position 1..=26
    // /// The AND and SHIFT will work for all u8, even non-alphabeticals.
    // /// Upper and lower case letters both map to the same position.
    // fn into(self) -> CharSet {
        // CharSet(self.bytes().fold(0, |acc, ch| acc | (1<<(ch & 31))))
    // }
// }

// use std::str::FromStr;
// #[derive(Debug, PartialEq, Eq)]
// struct CharSetError;

// impl FromStr for CharSet {
    // type Err = CharSetError;

    // fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ok(Self(s.bytes().fold(0, |acc, ch| acc | (1<<(ch & 31)))))
    // }
// }

#[cfg(test)]
mod test {
    use super::CharSet;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BAN: CharSet = CharSet::from("BAN");
        static ref BANG: CharSet = CharSet::from("BANG");
        static ref BANANA: CharSet = CharSet::from("BANANA");
        static ref BARN: CharSet = CharSet::from("BARN");
        static ref RAIN: CharSet = CharSet::from("RAIN");
        static ref RETAIN: CharSet = CharSet::from("RETAIN");
        static ref ZESTIER: CharSet = CharSet::from("ZESTIER");
    }

    #[test]
    fn test_from() {
        assert_eq!(CharSet(0u32), CharSet::from(""));
        assert_eq!(CharSet(0xeu32), CharSet::from("CAB"));
        assert_eq!(CharSet(0xeu32), CharSet::from("abc"));
    }

    #[test]
    fn test_contains() {
        assert!(CharSet(0xfu32).contains(CharSet(0x8u32)));
        assert!(!CharSet(0x10u32).contains(CharSet(0x8u32)));
        assert!(RETAIN.contains(*RAIN));
        assert!(!RAIN.contains(*RETAIN));
        assert!(!BANANA.contains(*BARN));
    }

    #[test]
    fn test_len() {
        assert_eq!(ZESTIER.len(), 6);
    }

    #[test]
    fn test_blanks_needed() {
        assert_eq!(RETAIN.blanks_needed(*RAIN), 0);
        assert_eq!(RAIN.blanks_needed(*RETAIN), 2);
    }

}

