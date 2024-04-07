// TODO: Put description here for doc

#[derive(Debug, PartialEq)]
pub struct CharSet(u32);

impl From<&str> for CharSet {
    /// Maps A..Z to a bit in position 1..=26
    /// The AND and SHIFT will work for all u8, even non-alphabeticals.
    /// Upper and lower case letters both map to the same position.
    fn from(word: &str) -> Self {
        Self(word.bytes().fold(0, |acc, ch| (acc | (1<<(ch & 0x1f)))))
    }
}

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
    fn basics() {
        assert_eq!(CharSet(0u32), CharSet::from(""));
        assert_eq!(CharSet(0xeu32), CharSet::from("CAB"));
        assert_eq!(CharSet(0xeu32), CharSet::from("abc"));
    }
}

