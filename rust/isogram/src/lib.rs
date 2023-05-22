use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .to_ascii_lowercase()
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| set.insert(c))
}
