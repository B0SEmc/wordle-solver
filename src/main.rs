use std::{
    fs::read_to_string,
    io::{Write, stdin, stdout},
};

const WORDLE_LENGTH: usize = 5;
const DICTIONARY: &str = "./src/words.txt";

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
    print!("{msg}");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    for i in input.chars() {
        if !i.is_ascii_alphabetic() && i != '*' {
            eprintln!("Invalid input (input must be alphabetic characters or *)!");
            std::process::exit(1);
        }
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

fn contain_all_missplaced(word: &[char], missplaced: &[Vec<char>]) -> bool {
    for (i, c) in word.iter().enumerate() {
        for pattern in missplaced {
            if pattern[i] != '*' && (!word.contains(&pattern[i]) || pattern[i] == *c) {
                return false;
            }
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

fn check_end(dictionnary: &[Vec<char>]) {
    if dictionnary.is_empty() {
        eprintln!("\nNo word found with this configuration!");
        std::process::exit(1);
    }
    if dictionnary.len() == 1 {
        println!(
            "\nThe only solution is: {}",
            dictionnary[0].iter().collect::<String>()
        );
        std::process::exit(0);
    }
}

fn get_best_word(dictionnary: &[Vec<char>]) -> &Vec<char> {
    let mut ideal: Vec<char> = vec!['A'; WORDLE_LENGTH];
    let mut word_score: Vec<i32> = vec![0; dictionnary.len()];
    for i in 0..WORDLE_LENGTH {
        let mut score: Vec<i32> = vec![0; 26];
        for word in dictionnary {
            score[word[i] as usize - 'A' as usize] += 1;
        }
        if let Some((index, _)) = score.iter().enumerate().max_by_key(|&(_, &val)| val) {
            ideal[i] = (index as u32 + 'A' as u32) as u8 as char;
        } else {
            eprintln!("An error occured when determining the best word to use!");
            std::process::exit(1);
        }
        for j in 0..dictionnary.len() {
            if dictionnary[j][i] == ideal[i] {
                word_score[j] += 1;
            }
        }
    }
    if let Some((index, _)) = word_score.iter().enumerate().max_by_key(|&(_, &val)| val) {
        &dictionnary[index]
    } else {
        eprintln!("An error occured when determining the best word to use!");
        std::process::exit(1);
    }
}

fn main() {
    let mut dictionnary = load_words(DICTIONARY);
    let pattern = read_user_input("Enter the pattern (t*Up* / nothing): ");
    if pattern.len() != WORDLE_LENGTH && !pattern.is_empty() {
        eprintln!("\nThe pattern must be {WORDLE_LENGTH} characters long!");
        std::process::exit(1);
    }
    dictionnary.retain(|word| is_correct_pattern(word, &pattern));
    check_end(&dictionnary);
    let mut user_line = read_user_input("Enter missplaced letters (*e*** / nothing): ");
    let mut missplaced = vec![];
    while !user_line.is_empty() {
        if user_line.len() != WORDLE_LENGTH && !user_line.is_empty() {
            eprintln!("\nThe missplaced letters pattern must be {WORDLE_LENGTH} characters long!");
            std::process::exit(1);
        }
        missplaced.push(user_line);
        dictionnary.retain(|word| contain_all_missplaced(word, &missplaced));
        check_end(&dictionnary);
        user_line = read_user_input("Enter missplaced letters (*e*** / nothing): ");
    }
    let invalid_letters = read_user_input("Enter invalid letters (AbC): ");
    dictionnary.retain(|word| does_not_contain_invalid_letters(word, &invalid_letters));
    check_end(&dictionnary);

    println!("\nAvailable words:");
    dictionnary
        .iter()
        .for_each(|word| println!("{}", word.iter().collect::<String>()));
    println!("\nBest word to try:");
    println!("{}", get_best_word(&dictionnary).iter().collect::<String>());
}
