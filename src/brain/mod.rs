use crate::{utils, views};
use std::env;

pub mod queries;

/// Brain Manager
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    queries::q_security_add_security_timestamps(queries::BRAIN_LOGON);
    views::start_menu_brain();

    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => queries::start_new_talk(),
            // "2" => queries::q_brain_search();
            // "3" => queries::q_brain_show_all(),
            "4" => queries::q_security_show_all(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
                utils::clear_screen()
            }
        }
        views::start_menu_brain();
    }
    utils::clear_screen();
}

/// Create folder Database (if does't exist) and verify the running operating system (OS) and return the correct path.
pub fn db_folder() -> String {
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        "\\Project\\Database\\".to_string()
    } else {
        "/Project/Database/".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _db_folder() {
        assert_eq!(
            db_folder(),
            if env::consts::OS.contains("windows") {
                "\\Project\\Database\\".to_string()
            } else {
                "/Project/Database/".to_string()
            }
        );
    }
}
