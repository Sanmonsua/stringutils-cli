
pub fn count_chars(s: &str) -> usize {
    s.chars().count()
}

pub fn split_words(s: &str) -> Vec<&str> {
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

