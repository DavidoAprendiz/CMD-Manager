use crate::{queries::security, utils::*, views};

pub fn main() {
    views::start_menu_security();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = get_user_input();
        match user_input.trim() {
            "1" => security::q_security_show_all(),
            _ => {
                if exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_security();
    }
    clear_screen();
}
