
fn reverse(s: &str) -> String {
    let chars = s.chars().rev();
    let reversed_string = String::from_iter(chars);
    reversed_string
}

fn is_palindrome(s: &str) -> bool {
    let reversed = reverse(s);
    reversed == *s
}

fn count_chars(s: &str) -> usize {
    s.chars().count()
}

fn split_words(s: &str) -> Vec<&str> {
    if count_chars(s) == 0 {
        let words: Vec<&str> = Vec::new();
        return words
    }

    let mut words = Vec::new();
    let mut last_pos = 0;
    let mut is_prev_space = false; 
    for (i, c) in s.char_indices() {
        if c == ' ' && !is_prev_space {
            words.push(&s[last_pos..i]);
            last_pos = i+1;
            is_prev_space = true;
        } else {
            is_prev_space = c == ' ';
        }
    }

    if !is_prev_space && last_pos < s.chars().count() {
        words.push(&s[last_pos..]);
    }

    words
}

fn count_words(s: &str) -> usize {
    let words = split_words(s);
    words.len()
}

fn unique_chars(s: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    for c in s.chars() {
        if !seen.insert(c) {
            return false;
        }
    }
    true
}

fn unique_words(s: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    let words = split_words(s);
    for w in words {
        if !seen.insert(w) {
            return false;
        }
    }
    true
}

fn main() {
    let my_input = String::from("abc abc");
    let reversed = reverse(&my_input);
    print!("my_input reversed is {reversed}");

    let palindrome = is_palindrome(&my_input);
    match palindrome {
        true => println!("is a palindrome."),
        false => println!("is not a palindrome"),
    }

    let char_count = count_chars(&my_input);
    let has_unique_chars = unique_chars(&my_input);

    match has_unique_chars {
        true => println!("Char count: {char_count} (All Unique)"),
        false => println!("Char count: {char_count} (Not Unique)"),
    }

    let word_count = count_words(&my_input);
    let has_unique_words = unique_words(&my_input);
    match has_unique_words {
        true => println!("Word count: {word_count} (All Unique)"),
        false => println!("Word count: {word_count} (Not Unique)"),
    }
}
