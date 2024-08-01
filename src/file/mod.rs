use crate::{queries, utils, views};
mod compare;
mod search;

/// File Manager
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    queries::security::q_security_add_timestamps(queries::FILE_LOGON);
    views::start_menu_file();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => search::main(),
            "2" => compare::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
                utils::clear_screen()
            }
        }
        views::start_menu_file();
    }
    utils::clear_screen();
}
