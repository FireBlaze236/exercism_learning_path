use std::collections::HashMap;
use std::collections::HashSet;

fn get_word_count_map(word: &str) -> HashMap<char, i32> {
    let mut word_count_map = HashMap::new();
    for c in word.chars() {
        if word_count_map.contains_key(&c) {
            let count = word_count_map.get(&c);
            let count = count.unwrap_or(&0);
            word_count_map.insert(c, count + 1);
        } else {
            word_count_map.insert(c, 1);
        }
    }

    word_count_map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word_count_map = get_word_count_map(word);
    for possible_anagram in possible_anagrams {
        if possible_anagram.len() == word.len() {
            let possible_anagram_word_count = get_word_count_map(possible_anagram);
            if possible_anagram_word_count == word_count_map {
                anagrams.insert(*possible_anagram);
            }
        }
    }

    return anagrams;
}
