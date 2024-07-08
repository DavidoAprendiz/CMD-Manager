mod brain; // Brain Manager
mod file; // File Manager
mod todo; // Todo Manager
mod utils; // Common functions
mod views; // Application views
mod web; // Web Manager

/// Application main loop
///
/// Start menu layout, begin loop, ask user input or exit program
fn main() {
    views::start_menu_main();
    start_db();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => todo::main(),
            "2" => file::main(),
            "3" => web::main(),
            "4" => brain::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
                utils::clear_screen()
            }
        }
        views::start_menu_main();
    }
}

/// Create tables (if not exists).
///
/// Add timestamp to TB_SECURITY
fn start_db() {
    brain::queries::q_security_create_table();
    brain::queries::q_brain_create_table();
    brain::queries::q_security_add_security_timestamps(brain::queries::SYSTEM_LOGON);
}
