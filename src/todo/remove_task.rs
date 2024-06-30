use crate::{todo, utils};
use std::fs;

/// Delete the task specified by the user.
pub fn remove_task() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                Remove a task!               #");
    println!("###############################################");

    utils::get_files_from_folder(todo::tasks_folder());

    println!("###############################################");
    println!("- Insert the name of the task to be removed  ");
    println!("     example:   16-06-2024-04_09_54.txt      \n");

    let user_input = utils::get_user_input();

    let path: String = {
        format_args!(
            "{}{}{}",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display(),
            todo::tasks_folder(),
            user_input
        )
        .to_string()
    };

    let file = fs::remove_file(path.trim());
    match file {
        Ok(_) => println!("\nFile removed: {path}\n"),
        Err(e) => println!("\nFailed to remove file.\n{e}\n"),
    }
}
