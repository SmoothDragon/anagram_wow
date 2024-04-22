use crate::{CharSet, CharMultiSet};

use itertools::Itertools;

#[derive(Debug)]
pub struct Query {
    pub required_set: CharSet,
    pub required: CharMultiSet,
    pub allowed_set: CharSet,
    pub allowed: CharMultiSet,
    pub length_min: usize,
    pub length_max: usize,
    pub blanks: usize,
}

impl From<&str> for Query {
    /// Parse an alphanumeric string
    /// Uppercase letters are required
    /// Lowercase letters are optional
    /// Number represents # of blanks allowed
    fn from(word: &str) -> Self {
        // if !word.chars().all(char::alphanumeric) { return None };
        let blanks = word.chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .fold(0, |acc, d| 10*acc+d)
            ;
        let required = word.chars()
            .filter(|ch| ch.is_ascii_uppercase())
            .join("")
            ;
        let allowed = word.to_uppercase().chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .join("")
            ;
        Self{
            required_set: CharSet::from(&*required),
            required: CharMultiSet::from(&*required),
            allowed_set: CharSet::from(&*allowed),
            allowed: CharMultiSet::from(&*allowed),
            length_min: required.len(),
            length_max: allowed.len()+blanks,
            blanks: blanks,
        }
    }
}
