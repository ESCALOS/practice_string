use std::collections::HashMap;

pub fn count_unique_chars_in_word(word: &str) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();

    for ch in word.chars().filter(|c| c.is_alphanumeric()) {
        let ch = ch.to_lowercase().next().unwrap();
        *map.entry(ch.to_string()).or_insert(0) += 1;
    }

    map
}