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

/// Clean the screen via cls (windows), clear.
pub fn clear_screen() {
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        match process::Command::new("cmd").args(["/c", "cls"]).status() {
            Ok(_) => (),
            Err(e) => println!("Failed to clear the screen.\n{e}\n"),
        }
    } else {
        println!("{}[2J", 27 as char);
    }
}

/// Boolean to call 'mainloop break' to exit the program.
pub fn exit_program(input: &str) -> bool {
    if input.trim().to_lowercase().starts_with('q') || input.trim().to_lowercase().starts_with('e')
    {
        clear_screen();
        println!("\n###############################################");
        println!("#                  Good bye!                  #");
        println!("###############################################\n");
        return true;
    }
    false
}

/// Get current directory plus folder (as String)
pub fn get_file_path(user_file: String) -> String {
    println!("{}", &user_file);
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        format!(
            "{}\\{user_file}",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display()
        )
    } else {
        format!(
            "{}/{user_file}",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display()
        )
    }
}

/// Check if folder 'Tasks' exists in root folder.
pub fn create_folder(path: String) {
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        match std::fs::create_dir_all(format!(
            "{}\\{path}\\",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display()
        )) {
            Ok(_) => {}
            Err(e) => println!("Failed to create folder 'Tasks'.\n{e}\n"),
        }
    } else {
        match std::fs::create_dir_all(format!(
            "{}/{path}/",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display()
        )) {
            Ok(_) => {}
            Err(e) => println!("Failed to create folder 'Tasks'.\n{e}\n"),
        }
    }
}

pub fn get_files_from_folder() {
    let path = get_file_path("".to_string());
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
