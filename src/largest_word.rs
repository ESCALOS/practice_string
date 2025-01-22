fn search(sentence: &str) -> Option<&str> {
    sentence.trim().split_whitespace().max_by_key(|word| word.len())
}

pub fn response(sentence: &str) {
    if let Some(largest_word) = search(sentence) {
        println!("La palabra mas grande es: {largest_word}");
    } else {
        println!("Error, no hay ninguna palabra");
    }
}