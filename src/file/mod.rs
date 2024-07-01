use crate::{utils, views};
mod compare;
mod search;

pub fn main() {
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
