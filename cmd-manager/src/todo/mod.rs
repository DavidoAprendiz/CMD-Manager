use chrono::{Local, Timelike};
use std::io::{self, Write};
use std::{fs, thread, time};

#[path = "../utils/mod.rs"]
mod utils;

pub fn main() {
    utils::clear_screen();
    start_menu();

    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();

        match user_input.trim() {
            "1" => add_task(),
            "2" => remove_task(),
            "3" => show_task(),
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
    get_tasks_from_folder();
    println!("###############################################");
    println!("#  Insert the name of the task to be removed  #");
    println!("#         example:      18_50                 #");
    println!("###############################################");

    let user_input = utils::get_user_input();

    let path = format_args!(
        "{}\\Tasks\\{}.txt",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
        user_input
    )
    .to_string();

    let file = fs::remove_file(path.trim());
    match file {
        Ok(_) => println!("\nFile removed: {path}\n"),
        Err(e) => println!("\nFailed to remove file.\n{e}\n"),
    }
}

fn _is_task_completed() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                  FUNC compl!                #");
    println!("###############################################");
}

/// Get all tasks in folder 'src\Tasks'
fn show_task() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                  All Tasks!                 #");
    println!("###############################################");
    get_tasks_from_folder();
    sleep_for(1500)
}

/// Create a new task with the user input. Generate field 'task_name' and 'task'.
fn add_task() {
    utils::clear_screen();
    println!("###############################################");
    println!("#              Create a new task!             #");
    println!("###############################################");
    println!("- Please insert the name of the task:");
    let mut task_name: String = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to get 'task_name'.\n");
    println!("- Please insert the task:");
    let mut task: String = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to get 'task'.\n");
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
    check_folder();
    let current_time = get_current_time();
    let path = format!(
        "{}\\Tasks\\{}.txt",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
        &current_time
    );
    let mut file = fs::File::create(path).expect("Failed to create text file!\n");
    match file.write_fmt(format_args!("{}\n{}", &user_task_name, &user_task)) {
        Ok(_) => (),
        Err(e) => println!("Failed to save task in file!\n{e}\n"),
    }
    println!("###############################################");
    println!("#                 TASK saved!                 #");
    println!("###############################################\n\n");
    show_task()
}

/// Iniciate a timer with milliseconds as input.
fn sleep_for(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

/// Calculate the Hours and Minutes from 'seconds_from_midnight'. Return a String and is used in 'save_task(), as exemple.'.
fn get_current_time() -> String {
    let hours = (Local::now().num_seconds_from_midnight() as f32) / 3600.00;
    let minutes = (&hours % 1.0) * 60.0;
    let hours = hours.abs() as i32;

    if hours < 10 && minutes < 10.0 {
        format!("0{:1.0}_0{:0.1}", hours, &minutes.to_string(),)
    } else if hours < 10 {
        format!("0{:1.0}_{:0.2}", hours, &minutes.to_string(),)
    } else if minutes < 10.0 {
        format!("{:0.0}_0{:0.1}", hours, &minutes.to_string(),)
    } else {
        format!("{:0.0}_{:0.2}", hours, &minutes.to_string(),)
    }
}

/// Loop to get all tasks in specific a folder 'src\Tasks'
fn get_tasks_from_folder() {
    check_folder();

    let path = format!(
        "{}\\Tasks",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
    );
    println!();
    match fs::read_dir(path) {
        Ok(tasks) => {
            for file in tasks {
                match file {
                    Ok(task) => {
                        println!("{:?}", task.file_name());
                    }
                    Err(e) => {
                        println!("Failed to read 'File Name' in folder 'Tasks'.\nError: {e}\n")
                    }
                }
            }
        }
        Err(e) => println!("Failed to read 'Tasks' folder content.\nError: {e}\n"),
    }
    println!();
}

/// Check if folder 'Tasks' exists in root folder.
fn check_folder() {
    match fs::create_dir_all(format!(
        "{}\\Tasks\\",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display()
    )) {
        Ok(_) => {}
        Err(e) => println!("Failed to create folder 'Tasks'.\n{e}\n"),
    }
}

/// Run the menu layout. Choose between 'welcome' and 'running' message.
fn start_menu() {
    println!("###############################################");
    println!("#                                             #");
    println!("#       cmd-manager  -  Your TODO list!       #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> New Task                         #");
    println!("#     '2' -> Remove Task                      #");
    println!("#     '3' -> Show Tasks                       #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}
