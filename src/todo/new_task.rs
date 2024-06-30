use crate::{todo, utils};
use std::{fs, io, io::Write};

/// Create a new task with the user input. Generate field 'task_name' and 'task'.
pub fn add_task() {
    utils::clear_screen();

    println!("###############################################");
    println!("#              Create a new task!             #");
    println!("###############################################");
    println!("- Please insert the task name:");

    let mut task_name: String = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to get 'task_name'.\n");
    println!("- Please insert the task description:");
    let mut task: String = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to get 'task description'.\n");
    println!("###############################################");
    save_task(task_name, task);
}

/// Save task to disk via User inputs.
fn save_task(user_task_name: String, user_task: String) {
    println!("\n###############################################");
    println!("#                                             #");
    println!(
        "\n  - {}\n  - {}\n",
        &user_task_name.trim(),
        &user_task.trim()
    );

    let path: String = {
        format!(
            "{}{}{}.txt",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display(),
            todo::tasks_folder(),
            utils::get_current_time()
        )
    };
    let mut file = fs::File::create(path).expect("Failed to create text file!\n");
    match file.write_fmt(format_args!(
        "Task Name: {}\nDescription:\n{}",
        &user_task_name, &user_task
    )) {
        Ok(_) => (),
        Err(e) => println!("Failed to save task in file!\n{e}\n"),
    }
    println!("###############################################");
    println!("#                 Task saved!                 #");
    println!("###############################################\n\n");
    todo::show_task::show_task()
}
