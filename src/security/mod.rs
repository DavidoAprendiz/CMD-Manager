use crate::{
    queries,
    utils::{clear_screen, exit_program, get_user_input, BLUE, CLOSE},
    views,
};
pub mod secure_db;

pub fn main() {
    views::start_menu_security();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = get_user_input();
        match user_input.trim() {
            "1" => queries::security::q_security_show_all(),
            "2" => secure_db::encrypt_db(queries::db_folder().as_str(), queries::DATABASE),
            "3" => secure_db::decrypt_db(queries::db_folder().as_str(), queries::DATABASE),
            _ => {
                if exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        println!("\n{BLUE}Press ENTER to continue...{CLOSE}");
        get_user_input();
        clear_screen();
        views::start_menu_security();
    }
    clear_screen();
}
