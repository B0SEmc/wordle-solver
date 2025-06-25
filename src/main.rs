use std::fs::read_to_string;

fn load_words(path: &str) -> Vec<Vec<char>> {
    let mut words = vec![];
    read_to_string(path)
        .unwrap()
        .lines()
        .for_each(|i| words.push(i.chars().collect()));
    words
}

fn main() {
    dbg!(load_words("words.txt"));
}
