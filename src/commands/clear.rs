use std::io;

use crossterm::{cursor, execute, terminal::{Clear, ClearType}};
use cursor::MoveTo;

pub fn clear_command() {
    execute!(io::stdout(), 
    Clear(ClearType::All), 
    MoveTo(0,0))
    .expect("Failed to clear screen");
}