#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::unix::fs::PermissionsExt;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        user_input = user_input.trim().to_string();

        if user_input == "exit" {
            break;
        } else if user_input.starts_with("echo ") {
            println!("{}", &user_input[5..]);
        } else if user_input.starts_with("type ") {
            let cmd = &user_input[5..];

            if ["echo", "exit", "type"].contains(&cmd) {
                println!("{} is a shell builtin", cmd);
            } else {
                let mut found = false;

                if let Ok(path_var) = env::var("PATH") {
                    for dir in env::split_paths(&path_var) {
                        let full_path = dir.join(cmd);

                        if let Ok(metadata) = std::fs::metadata(&full_path) {
                            let permissions = metadata.permissions();

                            // Check execute bits
                            if permissions.mode() & 0o111 != 0 {
                                println!("{} is {}", cmd, full_path.display());
                                found = true;
                                break;
                            }
                        }
                    }
                }

                if !found {
                    println!("{}: not found", cmd);
                }
            }
        } else {
            println!("{}: command not found", user_input);
        }
    }
}