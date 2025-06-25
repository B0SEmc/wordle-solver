use std::{
    fs::read_to_string,
    io::{Write, stdin, stdout},
};

const WORDLE_LENGTH: usize = 5;

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
    if input.len() != WORDLE_LENGTH && !input.is_empty() {
        eprintln!("The pattern must be {} characters long!", WORDLE_LENGTH);
        std::process::exit(1);
    }
    input.chars().map(|c| c.to_ascii_uppercase()).collect()
}

fn is_correct_pattern(word: &[char], pattern: &[char]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for (i, c) in word.iter().enumerate() {
        if pattern[i] != '*' && pattern[i] != *c {
            return false;
        }
    }
    true
}

fn contain_all_missplaced(word: &[char], missplaced: &[char]) -> bool {
    for c in missplaced {
        if !word.contains(c) {
            return false;
        }
    }
    true
}

fn does_not_contain_invalid_letters(word: &[char], invalid_letters: &[char]) -> bool {
    for c in invalid_letters {
        if word.contains(c) {
            return false;
        }
    }
    true
}

fn main() {
    let mut dictionnary = load_words("./src/words.txt");
    let pattern = read_user_input("Enter the pattern (t*Up*): ");
    dictionnary.retain(|word| is_correct_pattern(word, &pattern));
    if dictionnary.is_empty() {
        eprintln!("No word found with this configuration!");
        std::process::exit(1);
    }
    if dictionnary.len() == 1 {
        println!(
            "The only solution is: {}",
            dictionnary[0].iter().collect::<String>()
        );
        return;
    }
    let missplaced = read_user_input("Enter missplaced letters (aBc): ");
    dictionnary.retain(|word| contain_all_missplaced(word, &missplaced));
    if dictionnary.is_empty() {
        eprintln!("No word found with this configuration!");
        std::process::exit(1);
    }
    if dictionnary.len() == 1 {
        println!(
            "The only solution is: {}",
            dictionnary[0].iter().collect::<String>()
        );
        return;
    }
    let invalid_letters = read_user_input("Enter invalid letters (AbC): ");
    dictionnary.retain(|word| does_not_contain_invalid_letters(word, &invalid_letters));
    if dictionnary.is_empty() {
        eprintln!("No word found with this configuration!");
        std::process::exit(1);
    }
    if dictionnary.len() == 1 {
        println!(
            "The only solution is: {}",
            dictionnary[0].iter().collect::<String>()
        );
        return;
    }

    println!();
    println!("Available words:");
    dictionnary
        .into_iter()
        .for_each(|word| println!("{}", word.iter().collect::<String>()));
}
