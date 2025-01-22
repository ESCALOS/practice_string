fn capitalize_words(sentence: &str) -> Vec<String> {
    sentence
        .split_whitespace()
        .enumerate()
        .map(|(index, word)| {
            let capitalized_word = word
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().collect::<String>()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>();

            let suffix = match index {
                0 => "primera",
                1 => "segunda",
                2 => "tercera",
                _ => "n-ésima",
            };

            format!("{} ({})", capitalized_word, suffix)
        })
        .collect()

}

pub fn response(sentence: &str) {
    let capitalized_words = capitalize_words(sentence);

    for word in capitalized_words {
        println!("{word}");
    }
}