use crate::{todo, utils, views};
use std::{fs, io, io::Write};

// Define constants for colors (change colors in utils.rs)
const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;

/// Create a new task with the user input. Generate field 'task_name' and 'task'.
pub fn add_task() {
    views::start_menu_todo_new();
    println!("{CYAN_UNDERLINE}Please insert the task name:{CLOSE}");

    let mut task_name: String = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("\x1b[0m\x1b[31;3mFailed to get 'task_name'.\x1b[0m\n");
    println!("\n{CYAN_UNDERLINE}Please insert the task description:{CLOSE}");
    let mut task: String = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("\x1b[0m\x1b[31;3mFailed to get 'task description'.\x1b[0m\n");
    println!("\n{BLUE}###############################################{CLOSE}");
    save_task(task_name, task);
}

/// Save task to disk via User inputs.
fn save_task(user_task_name: String, user_task: String) {
    println!(
        "\n{CYAN_UNDERLINE}Task Name ->{CLOSE}  {}\n{CYAN_UNDERLINE}Task Description ->{CLOSE} {}\n",
        &user_task_name.trim(),
        &user_task.trim()
    );

    let path: String = {
        format!(
            "{}{}{}.txt",
            std::env::current_dir()
                .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n")
                .display(),
            todo::tasks_folder(),
            utils::get_current_time()
        )
    };
    let mut file =
        fs::File::create(path).expect("\x1b[0m\x1b[31;3mFailed to create text file!\x1b[0m");
    match file.write_fmt(format_args!(
        "Task Name: {}\nDescription:\n{}",
        &user_task_name, &user_task
    )) {
        Ok(_) => (),
        Err(e) => println!("\x1b[0m\x1b[31;3mFailed to save task in file!\n{e}\x1b[0m\n"),
    }
    views::start_menu_todo_saved();
    println!("{BLUE}Press ENTER to continue...{CLOSE}");
    utils::get_user_input();
}
