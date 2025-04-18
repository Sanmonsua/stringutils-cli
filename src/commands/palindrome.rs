
use super::reverse;

fn palindrome(s: &str) -> bool {
    let reversed = reverse::reverse(s);
    reversed == *s
}

pub fn run(s: Option<String>) {
    match s {
        None => println!("No argument has been passed."),
        Some(value) => {
            let is_palindrome = palindrome(&value);
            match is_palindrome {
                false => println!("Is not a palindrome"),
                true =>  println!("Yes! This word is a palindrome"),
            }
        }
    }
}

