use chrono::Local;
use std::{env, io, process};

/// Get user input for 'Menus'.
pub fn get_user_input() -> String {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => user_input.trim().to_string(),
        Err(_) => {
            user_input = String::from("");
            user_input
        }
    }
}

/// Clean the screen via clear.
pub fn clear_screen() {
    if env::consts::OS.contains("windows") {
        match process::Command::new("cmd").args(["/c", "cls"]).status() {
            Ok(_) => (),
            Err(e) => println!("Failed to clear the screen.\n{e}\n"),
        }
    } else {
        match process::Command::new("clear").status() {
            Ok(_) => (),
            Err(e) => println!("Failed to clear the screen.\n{e}\n"),
        }
    }
}

/// Boolean to call 'mainloop break' to exit the program.
pub fn exit_program(input: &str) -> bool {
    if input.trim().to_lowercase().starts_with('q') || input.trim().to_lowercase().starts_with('e')
    {
        clear_screen();
        println!("\n###############################################");
        println!("#                  Exiting...                 #");
        println!("###############################################\n");
        return true;
    }
    false
}

/// Get Operating System paths
pub fn get_os() -> String {
    if env::consts::OS.contains("windows") {
        "\\".to_string()
    } else {
        "/".to_string()
    }
}

/// Get current directory plus folder (as String)
pub fn get_file_path(user_file: String) -> String {
    format!(
        "{}{}{}",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
        get_os(),
        user_file
    )
}

/// Check if project folder exists in root folder.
pub fn create_folder(path: String) {
    match std::fs::create_dir_all(format!(
        "{}{}{}",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
        path,
        get_os()
    )) {
        Ok(_) => {}
        Err(e) => println!("Failed to create folder '{path}'.\n{e}\n"),
    }
}

// Get all files from a specific folder
pub fn get_files_from_folder(user_path: String) {
    let path = get_file_path(user_path);
    match std::fs::read_dir(path) {
        Ok(dirs) => {
            for files in dirs {
                match files {
                    Ok(file) => {
                        if file.path().is_file() {
                            println!("{:#?}", file.file_name());
                        }
                    }
                    Err(e) => {
                        println!(
                            "Failed to read the name of the files in current folder.\nError: {e}\n"
                        )
                    }
                }
            }
        }
        Err(e) => println!("Failed to read the current folder.\nError: {e}\n"),
    }
    println!();
}

/// Calculate the Hours and Minutes from 'seconds_from_midnight'. Return a String and is used in 'save_task(), as example.'.
pub fn get_current_time() -> String {
    Local::now().format("%d-%m-%Y-%H_%M_%S").to_string()
}
