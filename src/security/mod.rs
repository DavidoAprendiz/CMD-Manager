use crate::{queries::security, utils, views};

pub fn main() {
    views::start_menu_security();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => security::q_security_show_all(),
            "2" => todo!(), //protect your db with password
            "3" => todo!(), //encrypt your db with GPG (symmetric keys)
            "4" => todo!(), //remove db password protection
            "5" => todo!(), //remove db encryption keys
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_security();
    }
    utils::clear_screen();
}
