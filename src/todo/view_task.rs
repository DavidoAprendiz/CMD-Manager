use crate::{todo, utils, views};
use std::{env, fs};

const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const CYAN_UNDERLINE_BOLD: &str = utils::CYAN_UNDERLINE_BOLD;
const ERRO: &str = utils::ERRO;

/// Function to view selected task
pub fn view_task() {
    utils::clear_screen();
    views::start_menu_todo_view();
    utils::get_files_from_folder(todo::tasks_folder());
    println!("{CYAN_UNDERLINE}Please insert the task name:{CLOSE}");
    let user_input = utils::get_user_input();
    let user_input: String = {
        if env::consts::OS.contains("windows") {
            format!("Project\\Tasks\\{}", user_input)
        } else {
            format!("Project/Tasks/{}", user_input)
        }
    };
    let path = utils::get_file_path(user_input);
    utils::clear_screen();
    println!(
        "{CYAN_UNDERLINE_BOLD}Task Path{BLUE} -> {CLOSE}{}\n\n",
        &path
    );
    match fs::read_to_string(path) {
        Ok(file) => {
            println!("{}", file);
        }
        Err(e) => {
            println!("{ERRO}Failed to view task: {}{CLOSE}\n", e);
        }
    }
    println!("{BLUE}Press ENTER to continue...{CLOSE}");
    utils::get_user_input();
    utils::clear_screen();
}
