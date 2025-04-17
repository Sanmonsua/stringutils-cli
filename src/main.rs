
fn reverse(s: &String) -> String {
    let chars = s.chars().rev();
    let reversed_string = String::from_iter(chars);
    reversed_string
}


fn is_palindrome(s: &String) -> bool {
    let reversed = reverse(s);
    reversed == *s
}


fn main() {
    let my_input = String::from("kayak");
    let reversed = reverse(&my_input);
    let palindrome = is_palindrome(&my_input);
    match palindrome {
        true => println!("my_input reversed is {reversed}, and is a palindrome"),
        false => println!("my_input reversed is {reversed}, and is not a palindrome"),
    }
}
