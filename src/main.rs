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
        }else if user_input.starts_with("type ") {
            let cmd = &user_input[5..].to_string();
            if ["echo", "exit", "type"].iter().any(|&s| s == cmd) {
                println!("{} is a shell builtin",cmd);
            }
            else{
                println!("{}: not found", cmd);
            }
        }else{
            println!("{}: command not found", user_input.trim());
        }
    }
}
