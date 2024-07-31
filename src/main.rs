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
/// Start menu layout, create tables, begin loop, ask user input or exit program
fn main() {
    views::start_menu_main();
    queries::first_config();
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
