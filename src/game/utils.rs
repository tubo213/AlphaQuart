use std::io::{self, Write};

pub fn get_user_input(prompt: &str) -> usize {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().expect("Please enter a valid number")
}
