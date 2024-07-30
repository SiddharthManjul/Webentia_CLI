use std::env::{self};
use std::io::{self, Write};
use std::path::Path;

use dirs;

use crossterm::style::{Print, ResetColor};
use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
};

pub fn print_current_dir_prompt() -> Result<bool, Box<dyn std::error::Error>> {
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

pub fn change_directory(dir: &str, previous_dir: &mut Option<String>) -> io::Result<()> {

    let current_dir = env::current_dir()?;

    let path = match dir {
        "~" => dirs::home_dir().unwrap_or_else(|| Path::new("/").to_path_buf()),
        "-" => Path::new(previous_dir.as_deref().unwrap_or("/")).to_path_buf(),
        _ => Path::new(dir).to_path_buf(),
    };

    if env::set_current_dir(&path).is_ok() {
        *previous_dir = Some(current_dir.to_string_lossy().into_owned());
    } else {
        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("Error changing directory to: {}\n", path.display())),
            ResetColor
        )?;
    }
    Ok(())
}