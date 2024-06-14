use std::{io, process};

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

/// Clean the screen via cls.
pub fn clear_screen() {
    match process::Command::new("cmd").args(["/c", "cls"]).status() {
        Ok(_) => (),
        Err(e) => println!("Failed to clear the screen.\n{e}\n"),
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
