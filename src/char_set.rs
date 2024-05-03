// TODO: Put description here for doc
use std::fmt;
use std::iter::repeat;
use std::ops::Sub;
use itertools::Itertools;
use bitintr::Popcnt;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharMultiSet(u128);

impl From<&str> for CharMultiSet {
    /// Maps A..Z to a bit in position 1..=26
    /// The AND and SHIFT will work for all u8, even non-alphabeticals.
    /// Upper and lower case letters both map to the same position.
    fn from(word: &str) -> Self {
        Self(word.bytes().fold(0, |acc, ch| (acc + (1<<(((ch as u128)& 0x1f) << 2)))))
    }
}

impl Sub for CharMultiSet {
    /// Removes letter count of Other from Self
    /// Fails if there are not enough letters of each type to remove.
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        let result = self.0.wrapping_sub(other.0);
        if result & Self::REP8 == 0 {
            return Some(Self(result))
        } else {
            return None
        }
    }
}

impl CharMultiSet {
    const REP8:u128 = 0x88888888888888888888888888888888u128;
    // const REP7:u128 = 0x77777777777777777777777777777777u128;
    // const REP1:u128 = 0x11111111111111111111111111111111u128;
    const REP0F:u64 = 0xf0f0f0f0f0f0f0fu64;

    pub fn contains(self, other: CharMultiSet) -> bool {
        (self.0.wrapping_sub(other.0)) & Self::REP8 == 0u128
    }

    pub fn blanks_needed(self, other: CharMultiSet) -> usize {
        let diff = self.0 + Self::REP8 - other.0;
        let mask1 = ((diff ^ Self::REP8) & Self::REP8) >> 3;
        let mask7 = mask1 * 7;
        let m = ((diff & mask7) ^ mask7) + mask1;
        // println!("YES\n{:#034x}\n{:#034x}\n{:#034x}\n{:#034x}\n", self.0, other.0, diff, m);
        let result = (m + (m>>64)) as u64;
        // let result = result.wrapping_mul(0x0101010101010101u64);
        // (result >> 56) as usize
        let result = ((result >> 4) & Self::REP0F) + (result & Self::REP0F) ;
        let result = (result >> 8) + result;
        let result = (result >> 16) + result;
        let result = (result >> 32) + result;
        (result & 0xff) as usize
    }

    pub fn len(self) -> usize {
        let result = (self.0 + (self.0>>64)) as u64;
        // let result = result.wrapping_mul(0x1111111111111111u64);
        // println!("{}", Self(result as u128));
        // (result >> 56) as usize
        let result = ((result >> 4) & Self::REP0F) + (result & Self::REP0F);
        let result = (result >> 8) + result;
        let result = (result >> 16) + result;
        let result = (result >> 32) + result;
        (result & 0xff) as usize
    }
}

impl fmt::Display for CharMultiSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            ('A'..='Z').enumerate()
            .map(|(ii, ch)| format!("{}", 
                repeat(ch).
                take(((self.0 >> ((ii+1)<<2)) & 0xf) as usize)
                .collect::<String>())
                )
            .join("")
        )
    }
}
    
impl fmt::Debug for CharMultiSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#034x} => {}", self.0,
            ('A'..='Z').enumerate()
            // .map(|ii| format!("{}", ((self.0 >> (ii<<2)) & 0xf)))
            .map(|(ii, ch)| format!("{}{}", ch, ((self.0 >> ((ii+1)<<2)) & 0xf) ))
            .join(" ")
        )
    }
}
    
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

impl From<CharMultiSet> for CharSet {
    fn from(x: CharMultiSet) -> Self {
        let mut y:u128 = x.0;
        y |= y >> 2;
        y |= y >> 1;
        y &= 0x11111111111111111111111111111111u128;
        y ^= y >> 6;
        y ^= y >> 3;
        y &= 0x000f000f000f000f000f000f000f000fu128;
        y ^= y >> 24;
        y ^= y >> 12;
        y &= 0x000000000000ffff000000000000ffffu128;
        y ^= y >> 48;
        CharSet(y as u32)
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
        write!(f, "{:#034b} => {}", self.0,
            ('A'..='Z').enumerate()
            .map(|(ii, ch)| format!("{}{}", ch, ((self.0 >> (ii+1)) & 0x1) ))
            .join(" ")
        )
    }
}
    
impl fmt::Display for CharSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            (1..=26u8)
            .filter(|ii| (self.0 >> ii) & 1 == 1)
            .map(|ii| format!("{}", (64+ii) as char))
            .join("")
        )
    }
}
    
#[cfg(test)]
mod test {
    use super::{CharMultiSet, CharSet};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BAN: CharMultiSet = CharMultiSet::from("BAN");
        static ref BANG: CharMultiSet = CharMultiSet::from("BANG");
        static ref BANANA: CharMultiSet = CharMultiSet::from("BANANA");
        static ref BARN: CharMultiSet = CharMultiSet::from("BARN");
        static ref RAIN: CharMultiSet = CharMultiSet::from("RAIN");
        static ref RETAIN: CharMultiSet = CharMultiSet::from("RETAIN");
        static ref ZESTIER: CharMultiSet = CharMultiSet::from("ZESTIER");
        static ref CS_BAN: CharSet = CharSet::from("BAN");
        static ref CS_BANG: CharSet = CharSet::from("BANG");
        static ref CS_BANANA: CharSet = CharSet::from("BANANA");
        static ref CS_BARN: CharSet = CharSet::from("BARN");
        static ref CS_RAIN: CharSet = CharSet::from("RAIN");
        static ref CS_RETAIN: CharSet = CharSet::from("RETAIN");
        static ref CS_ZESTIER: CharSet = CharSet::from("ZESTIER");
    }

    #[test]
    fn test_char_multiset_from_str() {
        println!("{}", CharMultiSet::from("A"));
        println!("{}", CharMultiSet::from("abc"));
        assert_eq!(CharMultiSet(0u128), CharMultiSet::from(""));
        assert_eq!(CharMultiSet(0x30u128), CharMultiSet::from("aAa"));
        assert_eq!(CharMultiSet(0x1110u128), CharMultiSet::from("CAB"));
        assert_eq!(CharMultiSet(0x1110u128), CharMultiSet::from("abc"));
    }

    #[test]
    fn test_contains_char_multiset() {
        assert!(CharMultiSet(0x7u128).contains(CharMultiSet(0x1u128)));
        assert!(CharMultiSet(0x20u128).contains(CharMultiSet(0x10u128)));
        assert!(!CharMultiSet(0x10u128).contains(CharMultiSet(0x20u128)));
        assert!(RETAIN.contains(*RAIN));
        assert!(!RAIN.contains(*RETAIN));
        assert!(!BANANA.contains(*BARN));
        assert!(!BAN.contains(*BARN));
        assert!(BARN.contains(*BAN));
    }

    #[test]
    fn test_len_char_multiset() {
        assert_eq!(ZESTIER.len(), 7);
        assert_eq!(BAN.len(), 3);
    }

    #[test]
    fn test_blanks_needed_char_multiset() {
        assert_eq!(BANG.blanks_needed(*BANANA), 3);
        assert_eq!(BANANA.blanks_needed(*BANG), 1);
    }


    #[test]
    fn test_char_set_from_str() {
        assert_eq!(CharSet(0u32), CharSet::from(""));
        assert_eq!(CharSet(0xeu32), CharSet::from("CAB"));
        assert_eq!(CharSet(0xeu32), CharSet::from("abc"));
    }

    #[test]
    fn test_contains_char_set() {
        assert!(CharSet(0xfu32).contains(CharSet(0x8u32)));
        assert!(!CharSet(0x10u32).contains(CharSet(0x8u32)));
        assert!(CS_RETAIN.contains(*CS_RAIN));
        assert!(!CS_RAIN.contains(*CS_RETAIN));
        assert!(!CS_BANANA.contains(*CS_BARN));
    }

    #[test]
    fn test_char_set_len() {
        assert_eq!(CS_ZESTIER.len(), 6);
    }

    #[test]
    fn test_blanks_needed_char_set() {
        assert_eq!(CS_RETAIN.blanks_needed(*CS_RAIN), 0);
        assert_eq!(CS_RAIN.blanks_needed(*CS_RETAIN), 2);
    }

    #[test]
    fn test_char_set_from_char_multiset() {
        assert_eq!(CharSet::from(CharMultiSet::from("ABCDEFGHIJKLMNOPQRSTUVWXYV")), CharSet::from("ABCDEFGHIJKLMNOPQRSTUVWXYV"));
        assert_eq!(CharSet::from(CharMultiSet::from("XXXYYYZZZZZZZ")), CharSet::from("XXXYYYZZZZZZZ"));
        assert_eq!(CharSet::from(CharMultiSet::from("BANANA")), *CS_BANANA);
    }
}

