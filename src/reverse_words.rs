fn get(sentence: &str) -> Option<String> {
    let trimmed_sentence = sentence.trim();

    if trimmed_sentence.is_empty() {
        return None;
    }

    let mut result = String::new();

    for (i,word) in trimmed_sentence.split_whitespace().enumerate() {
        if i > 0 {
            result.push(' ');
        }

        result.push_str(&word.chars().rev().collect::<String>());
    }

    Some(result)
}

pub fn response(sentence: &str) {

    let empty_result = get(sentence);
    if let Some(reversed) = empty_result {
        println!("{}", reversed);
    } else {
        println!("La cadena está vacía");
    }
}