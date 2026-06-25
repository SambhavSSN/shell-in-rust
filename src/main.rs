#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn find_executable(cmd: &str) -> Option<std::path::PathBuf> {
    if let Ok(path_var) = env::var("PATH") {
        for dir in env::split_paths(&path_var) {
            let full_path = dir.join(cmd);

            if let Ok(metadata) = std::fs::metadata(&full_path) {
                if metadata.permissions().mode() & 0o111 != 0 {
                    return Some(full_path);
                }
            }
        }
    }

    None
}

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
            } else if let Some(path) = find_executable(cmd) {
                println!("{} is {}", cmd, path.display());
            } else {
                println!("{}: not found", cmd);
            }
        } else {
            // Split command into program + arguments
            let parts: Vec<&str> = user_input.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            let program = parts[0];

            if find_executable(program).is_some() {
                let _ = Command::new(program)
                    .args(&parts[1..])
                    .status();
            } else {
                println!("{}: command not found", program);
            }
        }
    }
}