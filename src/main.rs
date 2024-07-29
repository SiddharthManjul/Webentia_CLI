mod commands;

use clap::{command, Arg};
use std::io;

use commands::cd::{print_current_dir_prompt, change_directory};
use commands::ls::ls_command;

fn main() {
    let mut _input = String::new();

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
            Some("~") => {
                let target_dir = "/";
                if let Err(e) = change_directory(target_dir) {
                    eprintln!("cd: {}", e);
                }
            },
            Some("cd") => {
                let target_dir = parts.next().unwrap_or("/");
                if let Err(e) = change_directory(target_dir) {
                    eprintln!("cd: {}", e);
                }
            },
            Some("ls") => ls_command(),
            Some("exit") => break,
            Some(cmd) => eprintln!("Unknown command: {}", cmd),
            None => {}

        }
    }
}