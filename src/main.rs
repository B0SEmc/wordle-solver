use std::{
    fs::read_to_string,
    io::{Write, stdin, stdout},
};

fn load_words(path: &str) -> Vec<Vec<char>> {
    let mut words = vec![];
    read_to_string(path)
        .unwrap()
        .lines()
        .for_each(|i| words.push(i.chars().map(|c| c.to_ascii_uppercase()).collect()));
    words
}

fn get_pattern() -> Vec<char> {
    let mut pattern = String::new();
    stdin().read_line(&mut pattern).unwrap();
    pattern = pattern.trim().to_string();
    pattern.chars().map(|c| c.to_ascii_uppercase()).collect()
}

fn does_not_contain_missplaced(word: &[char], missplaced: &[char]) -> bool {
    for c in missplaced {
        if !word.contains(c) {
            return false;
        }
    }
    true
}

fn is_valid_word(
    word: &[char],
    pattern: &[char],
    missplaced: &[char],
    invalid_letters: &[char],
) -> bool {
    if !does_not_contain_missplaced(word, missplaced) {
        return false;
    }
    for (i, c) in word.iter().enumerate() {
        if invalid_letters.contains(c) || pattern[i] != '*' && pattern[i] != *c {
            return false;
        }
    }
    true
}

fn main() {
    let mut dictionnary = load_words("./src/words.txt");
    print!("Enter the pattern (A**D*): ");
    stdout().flush().unwrap();
    let pattern = get_pattern();
    print!("Enter missplaced letters (ABC): ");
    stdout().flush().unwrap();
    let missplaced = get_pattern();
    print!("Enter invalid letters (ABC): ");
    stdout().flush().unwrap();
    let invalid_letters = get_pattern();

    dictionnary.retain(|i| is_valid_word(i, &pattern, &missplaced, &invalid_letters));
    println!();
    println!("Available words:");
    dictionnary
        .into_iter()
        .for_each(|word| println!("{}", word.iter().collect::<String>()));
}
