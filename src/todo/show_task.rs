use crate::{brain::queries, utils, views};

/// Get all tasks in folder 'src\Tasks'
pub fn show_task() {
    utils::clear_screen();
    views::start_menus("Show all tasks!");
    queries::q_todo_show_all();
    println!(
        "\n{}###############################################\nPress ENTER to continue...{}",
        utils::BLUE,
        utils::CLOSE
    );
    utils::get_user_input();
}
