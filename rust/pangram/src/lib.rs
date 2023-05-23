use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    HashSet::<u8>::from_iter(b'a'..=b'z')
        .is_subset(&HashSet::from_iter(sentence.to_ascii_lowercase().bytes()))
}
