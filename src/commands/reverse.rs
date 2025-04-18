
pub fn reverse(s: &str) -> String {
    let chars = s.chars().rev();
    let reversed_string = String::from_iter(chars);
    reversed_string
}

pub fn run(s: Option<String>) {
    match s {
        None => println!("No argument has been passed."),
        Some(value) => {
            let reversed = reverse(&value);
            println!("Reversed string is {reversed}")
        }
    }
}
