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

fn read_user_input(msg: &str) -> Vec<char> {
    let mut input = String::new();
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input.chars().map(|c| c.to_ascii_uppercase()).collect()
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
    let pattern = read_user_input("Enter the pattern (t*Up*): ");
    let missplaced = read_user_input("Enter missplaced letters (aBc): ");
    let invalid_letters = read_user_input("Enter invalid letters (AbC): ");

    dictionnary.retain(|i| is_valid_word(i, &pattern, &missplaced, &invalid_letters));
    println!();
    println!("Available words:");
    dictionnary
        .into_iter()
        .for_each(|word| println!("{}", word.iter().collect::<String>()));
}
