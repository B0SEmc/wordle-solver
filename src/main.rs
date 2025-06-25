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

fn does_contain_letter(word: &[char], letters: &Vec<char>) -> bool {
    for letter in letters {
        if !word.contains(letter) {
            return false;
        }
    }
    true
}

fn main() {
    let dictionnary = load_words("./src/words.txt");
    print!("Enter the pattern (A**D*): ");
    stdout().flush().unwrap();
    let pattern = get_pattern();
    print!("Enter missplaced letters (ABC): ");
    stdout().flush().unwrap();
    let missplaced = get_pattern();
    let mut correct_words: Vec<Vec<char>> = vec![];

    for word in dictionnary.into_iter() {
        let mut matches = true;
        for (i, c) in word.iter().enumerate() {
            if pattern[i] != '*' && pattern[i] != *c {
                matches = false;
                break;
            }
        }
        if matches {
            correct_words.push(word);
        }
    }
    correct_words.retain(|i| does_contain_letter(i, &missplaced));
    println!();
    println!("Available words:");
    correct_words
        .into_iter()
        .for_each(|word| println!("{}", word.iter().collect::<String>()));
}
