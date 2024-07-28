use crate::{queries::todo, utils, views};
use std::io;

/// Create a new task with the user input. Generate field 'task_name' and 'task'.
pub fn add_task() {
    views::start_menus("Create a new task!");

    println!(
        "{}Please insert the task name:{}",
        utils::CYAN_UNDERLINE,
        utils::CLOSE
    );
    let mut task_name: String = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("\x1b[0m\x1b[31;3mFailed to get 'task_name'.\x1b[0m\n");

    println!(
        "\n{}Please insert the task description:{}",
        utils::CYAN_UNDERLINE,
        utils::CLOSE
    );
    let mut task: String = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("\x1b[0m\x1b[31;3mFailed to get 'task description'.\x1b[0m\n");

    todo::q_todo_add_task(task_name, task);
    views::start_menus("Task saved!");
    println!(
        "\n{}Press ENTER to continue...{}",
        utils::BLUE,
        utils::CLOSE
    );
    utils::get_user_input();
}
