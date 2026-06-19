#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        //user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        println!("{}: command not found", user_input.trim());
    }
}
