// TODO: Put description here for doc

use bitintr::Popcnt;

#[derive(Debug, PartialEq, Clone, Copy)]
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
        assert!(CharSet::from("RETAIN").contains(CharSet::from("RAIN")));
        assert!(!CharSet::from("RAIN").contains(CharSet::from("RETAIN")));
    }

    #[test]
    fn test_len() {
        assert_eq!(CharSet::from("ZESTIER").len(), 6);
    }
}

