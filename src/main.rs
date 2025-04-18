
fn reverse(s: &String) -> String {
    let chars = s.chars().rev();
    let reversed_string = String::from_iter(chars);
    reversed_string
}

fn is_palindrome(s: &String) -> bool {
    let reversed = reverse(s);
    reversed == *s
}

fn count_chars(s: &String) -> usize {
    s.chars().count()
}

fn split_words(s: &String) -> Vec<&str> {
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
            last_pos = i;
            is_prev_space = true;
        } else {
            is_prev_space = c == ' ';
        }
    }

    if !is_prev_space {
        words.push(&s[last_pos..]);
    }

    words
}

fn count_words(s: &String) -> usize {
    let words = split_words(s);
    words.len()
}

fn main() {
    let my_input = String::from("kayak  and lust");
    let reversed = reverse(&my_input);
    let palindrome = is_palindrome(&my_input);
    let char_count = count_chars(&my_input);
    let word_count = count_words(&my_input);
    match palindrome {
        true => println!("my_input reversed is {reversed}, and is a palindrome. Len: {char_count}. Words: {word_count}"),
        false => println!("my_input reversed is {reversed}, and is not a palindrome. Len: {char_count}. Words: {word_count}"),
    }
}
