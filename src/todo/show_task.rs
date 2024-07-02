use crate::{todo, utils, views};

const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;

/// Get all tasks in folder 'src\Tasks'
pub fn show_task() {
    utils::clear_screen();
    views::start_menu_todo_all();
    utils::get_files_from_folder(todo::tasks_folder());
    println!("{BLUE}Press Enter to continue...{CLOSE}");
    utils::get_user_input();
}
