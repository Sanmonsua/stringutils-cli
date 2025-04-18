
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
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
