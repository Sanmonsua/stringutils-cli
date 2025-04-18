use crate::utils;

fn count_words(s: &str) -> usize {
    let words = utils::split_words(s);
    words.len()
}

pub fn run(s: Option<String>, chars: bool, words: bool) {
    match s {
        None => println!("No argument has been passed."),
        Some(value) => {
            if words {
                let word_count = count_words(&value);
                println!("Word count: {word_count}");
            } else if chars {
                let char_count = utils::count_chars(&value);
                println!("Character count: {char_count}");
            }
        }
    }
}
