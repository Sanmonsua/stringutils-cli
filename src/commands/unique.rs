use crate::utils;

fn unique_chars(s: &str) -> (std::collections::HashSet<char>, bool) {
    let mut seen = std::collections::HashSet::new();
    let mut are_all_unique = true;
    for c in s.chars() {
        let inserted = seen.insert(c);
        are_all_unique =  are_all_unique && inserted
    }
    (seen, are_all_unique)
}

fn unique_words(s: &str) -> (std::collections::HashSet<&str>, bool) {
    let mut seen = std::collections::HashSet::new();
    let words = utils::split_words(s);
    let mut are_all_unique = true;
    for w in words {
        are_all_unique =  are_all_unique && seen.insert(w)
    }
    (seen, are_all_unique)
}

pub fn run(s: Option<String>, chars: bool, words: bool) {
    match s {
        None => println!("No argument has been passed."),
        Some(value) => {
            if words {
                let (unique_words_set, are_unique) = unique_words(&value);
                println!("Unique words: {:?}", unique_words_set);
                if are_unique {
                    println!("All words are unique");
                } else {
                    println!("Not all words are unique");
                }
            } else {
                if chars {
                    let (unique_chars_set, are_unique) = unique_chars(&value);
                    println!("Unique characters: {:?}", unique_chars_set);
                    if are_unique {
                        println!("All characters are unique");
                    } else {
                        println!("Not all characters are unique");
                    }
                }
            }
        }
    }
}
