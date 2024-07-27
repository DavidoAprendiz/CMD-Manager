use crate::{brain::queries, utils, views};

/// Delete the task specified by the user.
pub fn remove_task() {
    utils::clear_screen();
    views::start_menus("Delete Task!");
    queries::q_todo_show_all();
    queries::q_todo_delete_task();
}
