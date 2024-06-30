use crate::{todo, utils};

/// Get all tasks in folder 'src\Tasks'
pub fn show_task() {
    utils::clear_screen();

    println!("###############################################");
    println!("#                  All Tasks!                 #");
    println!("###############################################");

    utils::get_files_from_folder(todo::tasks_folder());
    todo::sleep_for(1500)
}
