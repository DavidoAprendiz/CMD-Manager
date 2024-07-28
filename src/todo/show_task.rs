use crate::{queries::todo, utils, views};

/// Get all tasks in folder 'src\Tasks'
pub fn show_task() {
    views::start_menus("Show all tasks!");
    todo::q_todo_show_all();
    println!(
        "\n{}###############################################\nPress ENTER to continue...{}",
        utils::BLUE,
        utils::CLOSE
    );
    utils::get_user_input();
}
