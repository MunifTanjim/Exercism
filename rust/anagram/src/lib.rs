use std::collections::HashSet;

fn get_sorted_word(word: &str) -> String {
    let mut word_chars = word.chars().collect::<Vec<char>>();
    word_chars.sort_unstable();
    word_chars.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = get_sorted_word(&word);

    possible_anagrams
        .iter()
        .filter_map(|p| {
            let w = p.to_lowercase();
            if w != word && sorted_word == get_sorted_word(&w) {
                Some(*p)
            } else {
                None
            }
        })
        .collect()
}
