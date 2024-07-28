use crate::{queries, queries::brain, utils, views};

pub mod talk;

/// Brain Manager
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    queries::security::q_security_add_security_timestamps(queries::BRAIN_LOGON);
    views::start_menu_brain();

    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        utils::clear_screen();
        match user_input.trim() {
            "1" => talk::start_new_talk(),
            "2" => brain::q_brain_search_talks(),
            "3" => brain::q_brain_delete_talk(),
            "4" => {
                brain::q_brain_show_all();
                println!("{}press ENTER to continue...{}", utils::BLUE, utils::CLOSE);
                utils::get_user_input();
            }
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_brain();
    }
    utils::clear_screen();
}
