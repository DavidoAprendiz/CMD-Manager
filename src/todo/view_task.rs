use crate::{todo, utils};
use std::{env, fs};

/// Function to view selected task
pub fn view_task() {
    utils::clear_screen();

    println!("###############################################");
    println!("#              View your tasks!               #");
    println!("###############################################");

    utils::get_files_from_folder(todo::tasks_folder());
    println!("- Please insert the task name:");
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
    println!("Task Path -> {}\n\n", &path);
    match fs::read_to_string(path) {
        Ok(file) => {
            println!("{}", file);
        }
        Err(e) => {
            println!("Failed to view task: {}", e);
        }
    }
    utils::get_user_input();
    utils::clear_screen();
}
