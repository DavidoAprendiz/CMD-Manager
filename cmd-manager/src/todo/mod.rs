use chrono::Local;
use std::io::{self, Write};
use std::{env, fs, thread, time};

use crate::utils;

pub fn main() {
    utils::clear_screen();
    utils::create_folder("Tasks".to_string());
    start_menu();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();

        match user_input.trim() {
            "1" => add_task(),
            "2" => remove_task(),
            "3" => show_task(),
            "4" => view_task(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        start_menu();
    }
    utils::clear_screen()
}

/// Delete the task specified by the user.
fn remove_task() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                Remove a task!               #");
    println!("###############################################");
    //get_tasks_from_folder();
    utils::get_files_from_folder("Tasks".to_string());
    println!("###############################################");
    println!("- Insert the name of the task to be removed  ");
    println!("     example:   16-06-2024-04_09_54.txt      \n");
    let user_input = utils::get_user_input();

    let operating_system = env::consts::OS;
    let operating_system = if operating_system.contains("windows") {
        "\\Tasks\\"
    } else {
        "/Tasks/"
    };

    let path: String = {
        format_args!(
            "{}{}{}",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display(),
            operating_system,
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

/// Get all tasks in folder 'src\Tasks'
fn show_task() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                  All Tasks!                 #");
    println!("###############################################");
    //get_tasks_from_folder();
    utils::get_files_from_folder("Tasks".to_string());
    sleep_for(1500)
}

/// Create a new task with the user input. Generate field 'task_name' and 'task'.
fn add_task() {
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

    let operating_system = env::consts::OS;
    let operating_system = if operating_system.contains("windows") {
        "\\Tasks\\"
    } else {
        "/Tasks/"
    };

    let path: String = {
        format!(
            "{}{}{}.txt",
            std::env::current_dir()
                .expect("Failed to access current directory.\n")
                .display(),
            operating_system,
            get_current_time()
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
    show_task()
}

/// Initiate a timer with milliseconds as input.
fn sleep_for(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

/// Calculate the Hours and Minutes from 'seconds_from_midnight'. Return a String and is used in 'save_task(), as example.'.
fn get_current_time() -> String {
    Local::now().format("%d-%m-%Y-%H_%M_%S").to_string()
}

/// Function to view selected task via 'cat'
fn view_task() {
    utils::clear_screen();
    println!("###############################################");
    println!("#              View your tasks!               #");
    println!("###############################################");
    //get_tasks_from_folder();
    utils::get_files_from_folder("Tasks".to_string());
    println!("- Please insert the task name:");
    let user_input = utils::get_user_input();

    let operating_system = env::consts::OS;
    let user_input: String = {
        if operating_system.contains("windows") {
            format!("Tasks\\{}", user_input)
        } else {
            format!("Tasks/{}", user_input)
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

/// Run the menu layout.
fn start_menu() {
    println!("###############################################");
    println!("#                                             #");
    println!("#                To-do Manager                #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> New Task                         #");
    println!("#     '2' -> Remove Task                      #");
    println!("#     '3' -> Show All Tasks                   #");
    println!("#     '4' -> View Task                        #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}
