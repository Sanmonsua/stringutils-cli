
fn reverse(s: &String) -> String {
    let chars = s.chars().rev();
    let reversed_string = String::from_iter(chars);
    reversed_string
}

fn main() {
    let my_input = String::from("Hello Santi");
    let reversed = reverse(&my_input);
    println!("my_input reversed is {reversed}");
}
