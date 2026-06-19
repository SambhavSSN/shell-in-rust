#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        //user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        user_input = user_input.trim().to_string();
        
        if user_input == "exit" {
            break;
        }else if user_input.starts_with("echo "){
            println!("{}",&user_input[5..]);
        }else{
            println!("{}: command not found", user_input.trim());
        }
    }
}
