use std::collections::HashSet;

fn is_anagram(word: &str, candidate: &str) -> bool {
    fn to_sorted_vec(s: &str) -> Vec<String> {
        let mut v = s
            .chars()
            .map(|c| c.to_lowercase().to_string())
            .collect::<Vec<String>>();
        v.sort();
        v
    }

    to_sorted_vec(word).eq(&to_sorted_vec(candidate))
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|a| is_anagram(word, a))
        .filter(|a| a.to_lowercase() != word.to_lowercase())
        .copied()
        .collect()
}
