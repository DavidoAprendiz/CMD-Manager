use crate::{queries, utils, views};
mod download;
mod web_req;

/// Web Manager
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    queries::security::q_security_add_timestamps(queries::WEB_LOGON);
    views::start_menu_web();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        utils::clear_screen();
        match user_input.trim() {
            "1" => download::main(),
            "2" => web_req::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_web();
    }
    utils::clear_screen();
}
