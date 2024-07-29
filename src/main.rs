mod brain; // Brain Manager
mod file; // File Manager
mod queries; // All Queries to DB
mod security; // Security Manager
mod todo; // Todo Manager
mod utils; // Common functions
mod views; // Application views
mod web; // Web Manager

/// Application main loop
///
/// Start menu layout, begin loop, ask user input or exit program
fn main() {
    views::start_menu_main();
    first_config();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => todo::main(),
            "2" => file::main(),
            "3" => web::main(),
            "4" => brain::main(),
            "5" => security::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_main();
    }
}

/// Create tables (if not exists).
/// Add timestamp to TB_SECURITY
fn first_config() {
    utils::create_folder(queries::db_folder());
    queries::security::q_security_create_table();
    queries::todo::q_todo_create_table();
    // TODO queries::file::q_file_create_table();
    // TODO queries::web::q_web_create_table();
    queries::brain::q_brain_create_table();
    queries::security::q_security_add_security_timestamps(queries::SYSTEM_LOGON);
}
