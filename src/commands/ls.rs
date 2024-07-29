use std::fs;

pub fn ls_command() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => println!("{}", e.file_name().into_string().unwrap()),
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}
