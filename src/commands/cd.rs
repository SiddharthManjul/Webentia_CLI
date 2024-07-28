use std::env::{self};
use std::io::{self, Write};
use std::path::Path;

use dirs;

use crossterm::style::{Print, ResetColor};
use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
};

pub fn execute_cd() {
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
            Some("exit") => break,
            Some(cmd) => eprintln!("Unknown command: {}", cmd),
            None => {}

        }
    }
    
}

fn print_current_dir_prompt() -> Result<bool, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("/").to_path_buf());
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Cyan),
        Print(format!("{} \n", current_dir.display())),
        ResetColor
    )?;
    io::stdout().flush()?;
    Ok(true)
}

fn change_directory(dir: &str) -> io::Result<()> {

    let path = match dir {
        "~" => dirs::home_dir().unwrap_or_else(|| Path::new("/").to_path_buf()),
        _ => Path::new(dir).to_path_buf(),
    };

    if let Err(e) = env::set_current_dir(&path) {
        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("Error changing directory: {}\n", e)),
            ResetColor
        )?;
    }
    Ok(())
}