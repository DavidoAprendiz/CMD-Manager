use crate::{queries::todo, views};

/// Delete the task specified by the user.
pub fn remove_task() {
    views::start_menus("Delete Task!");
    todo::q_todo_show_all();
    todo::q_todo_delete_task();
}
