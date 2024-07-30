mod commands;

use std::io;

use commands::cd::{print_current_dir_prompt, change_directory};
use commands::ls::ls_command;
use commands::pwd::pwd_command;
use commands::clear::clear_command;

fn main() {
    let mut _input = String::new();
    let mut previous_dir:Option<String> = None;

    loop {
        let _ = print_current_dir_prompt();

        // Read User Input
        _input.clear();
        io::stdin().read_line(&mut _input).unwrap();

        // Trim and Parse the Command
        let trimmed_input = _input.trim();
        let mut parts = trimmed_input.split_whitespace();
        let command = parts.next();

        match command {
            Some("cd") => {
                let target_dir = parts.next().unwrap_or("/");
                if let Err(e) = change_directory(target_dir, &mut previous_dir) {
                    eprintln!("cd: {}", e);
                }
            },
            Some("ls") => {
                let show_hidden = parts.next() == Some("-a");
                ls_command(show_hidden);
            },
            Some("pwd") => pwd_command(),
            Some("clear") => clear_command(),
            Some("exit") => break,
            Some(cmd) => eprintln!("Unknown command: {}", cmd),
            None => {}

        }
    }
}