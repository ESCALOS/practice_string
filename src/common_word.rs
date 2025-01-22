use std::collections::HashMap;

fn analyze(sentence: &str) -> Option<(String, usize)> {

    if sentence.trim().is_empty() {
        return None;
    }

    let mut word_count: HashMap<String, usize> = HashMap::new();

    for word in sentence.split_whitespace() {
        let clean_word = word
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        if !clean_word.is_empty() {
            *word_count.entry(clean_word).or_insert(0) += 1;
        }
    }

    word_count
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(word, &count)| (word.clone(), count))

}

pub fn response(sentence: &str) {
    match analyze(sentence) {
        Some((word, count)) => println!("La palabra más común es {word} con {count} apariciones."),
        None => println!("La frase está vacía o no contiene palabras.")
    }
}