use std::collections::HashMap;

fn build_char_count_map(s: &str) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for ch in s.chars().filter(|c| c.is_alphanumeric()) {
        let ch  = ch.to_lowercase().next().unwrap();
        *char_count.entry(ch).or_insert(0) += 1;
    }

    char_count
}

pub fn validate(s1: &str, s2: &str) -> bool {
    let map1 = build_char_count_map(s1);
    let map2 = build_char_count_map(s2);
    
    map1 == map2
}