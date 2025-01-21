fn verify(word :&str) -> bool {
    let normalized_word = word.to_lowercase();
    let reversed_word: String = normalized_word.chars().rev().collect();

    normalized_word == reversed_word
}

pub fn response(word: &str) {
    let is_palindromo = verify(word);

    if is_palindromo {
        println!("Es palíndromo");
    } else {
        println!("No es palíndromo");
    }
}