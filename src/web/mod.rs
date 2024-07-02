use crate::{utils, views};
mod download;
mod web_req;

pub fn main() {
    views::start_menu_web();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => download::main(),
            "2" => web_req::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
                utils::clear_screen()
            }
        }
        views::start_menu_web();
    }
    utils::clear_screen();
}
