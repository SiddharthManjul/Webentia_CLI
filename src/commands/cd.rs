use std::env;
use std::io::{self, Write};
use std::path::Path;

use crossterm::style::{Print, ResetColor};
use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
};

pub fn execute_cd() {
    let mut _input = String::new();

    loop {
        print_current_dir_prompt();

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
                if let Err(e) = change_directory(target_dir) {
                    eprintln!("cd: {}", e);
                }
            }
            Some("exit") => break,
            Some(cmd) => eprintln!("Unknown command: {}", cmd),
            None => {}

        }
    }
    
}

fn print_current_dir_prompt() {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("/").to_path_buf());
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Cyan),
        Print(format!("{} \n", current_dir.display())),
        ResetColor
    ).unwrap();
    io::stdout().flush().unwrap();
}

fn change_directory(dir: &str) -> io::Result<()> {
    let new_dir = Path::new(dir);
    if new_dir.is_dir() {
        env::set_current_dir(new_dir)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"));
    }
    Ok(())
}