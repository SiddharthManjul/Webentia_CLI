use std::fs;

pub fn ls_command(show_hidden: bool) {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        let file_name = e.file_name();
                        let file_name_str = file_name.to_string_lossy();
                        if !show_hidden && file_name_str.starts_with('.') {
                            continue;
                        }
                        println!("{}", file_name_str);
                    },
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}