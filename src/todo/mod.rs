use crate::{brain, utils, views};
mod new_task;
mod remove_task;
mod search_task;
mod show_task;

/// Todo Manager
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    brain::queries::q_security_add_security_timestamps(brain::queries::TODO_LOGON);
    views::start_menu_todo();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => new_task::add_task(),
            "2" => remove_task::remove_task(),
            "3" => search_task::search_task(),
            "4" => show_task::show_task(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_todo();
    }
    utils::clear_screen();
}
